use ic_cdk::api;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use icrc_ledger_types::icrc1::account::Account;
use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use std::cell::RefCell;

use crate::icrc2::{approve, transfer};
use crate::types::{Permit, StorableAccount, TransferArgs, TxResult};

// Define the stable memory type
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Store nonces for each account persistently in stable memory
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static NONCES: RefCell<StableBTreeMap<StorableAccount, u64, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        ));
}

// Get the current nonce for an account
fn get_nonce(owner: &Account) -> u64 {
    NONCES.with(|n| n.borrow().get(&StorableAccount(owner.clone())).unwrap_or(0))
}

// Increment the nonce for an account after a successful permit
fn increment_nonce(owner: &Account) {
    NONCES.with(|n| {
        let mut map = n.borrow_mut();
        let key = StorableAccount(owner.clone());
        let current = map.get(&key).unwrap_or(0);
        map.insert(key, current + 1);
    });
}

/// Verify the permit signature and execute the transfer if valid
pub fn verify_and_transfer(p: Permit, to: Account) -> TxResult {
    // 1️⃣ Check expiration
    let now = api::time() / 1_000_000_000;
    if now > p.deadline {
        return TxResult::Err("Permit expired".into());
    }

    // 2️⃣ Verify nonce
    let current_nonce = get_nonce(&p.owner);
    if p.nonce != current_nonce {
        return TxResult::Err("Invalid nonce".into());
    }

    // 3️⃣ Build the permit message hash (ICRC30 domain)
    let message = format!(
        "ICRC30 Permit:\nowner={:?}\nspender={:?}\nvalue={}\nnonce={}\ndeadline={}",
        p.owner, p.spender, p.value, p.nonce, p.deadline
    );
    let hash = Sha256::digest(message.as_bytes());

    // 4️⃣ Verify ECDSA signature (SECP256K1)
    // ⚠️ NOTE: In production, the owner's public key should be registered or derived securely.
    let owner_pubkey: [u8; 33] = [0; 33];
    let Ok(vk) = VerifyingKey::from_sec1_bytes(&owner_pubkey) else {
        return TxResult::Err("Invalid owner public key".into());
    };
    let Ok(sig) = Signature::from_slice(&p.signature) else {
        return TxResult::Err("Invalid signature format".into());
    };
    if vk.verify(&hash, &sig).is_err() {
        return TxResult::Err("Signature verification failed".into());
    }

    // 5️⃣ Update nonce after successful verification
    increment_nonce(&p.owner);

    // 6️⃣ Record the approval and execute the transfer
    approve(p.owner.clone(), p.spender.clone(), p.value);

    transfer(TransferArgs {
        from: p.owner,
        to,
        amount: p.value,
    })
}
