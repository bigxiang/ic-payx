use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

use crate::types::{StorableAccount, TransferArgs, TxResult};
use icrc_ledger_types::icrc1::account::Account;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static BALANCES: RefCell<StableBTreeMap<StorableAccount, u128, Memory>> =
        RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
            )
        );

    static ALLOWANCES: RefCell<StableBTreeMap<(StorableAccount, StorableAccount), u128, Memory>> =
        RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
            )
        );
}

pub fn transfer(args: TransferArgs) -> TxResult {
    let from_key = StorableAccount(args.from.clone());
    let to_key = StorableAccount(args.to.clone());

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let from_balance = balances.get(&from_key).unwrap_or(0);
        if from_balance < args.amount {
            return TxResult::Err("Insufficient balance".into());
        }
        balances.insert(from_key.clone(), from_balance - args.amount);
        let to_balance = balances.get(&to_key).unwrap_or(0);
        balances.insert(to_key.clone(), to_balance + args.amount);
        TxResult::Ok
    })
}

pub fn approve(owner: Account, spender: Account, value: u128) {
    ALLOWANCES.with(|a| {
        a.borrow_mut()
            .insert((StorableAccount(owner), StorableAccount(spender)), value);
    });
}

pub fn allowance(owner: &Account, spender: &Account) -> u128 {
    ALLOWANCES.with(|a| {
        a.borrow()
            .get(&(
                StorableAccount(owner.clone()),
                StorableAccount(spender.clone()),
            ))
            .unwrap_or(0)
    })
}

pub fn balance_of(owner: &Account) -> u128 {
    BALANCES.with(|b| b.borrow().get(&StorableAccount(owner.clone())).unwrap_or(0))
}

pub fn mint(to: Account, amount: u128) {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let key = StorableAccount(to);
        let prev = balances.get(&key).unwrap_or(0);
        balances.insert(key, prev + amount);
    });
}
