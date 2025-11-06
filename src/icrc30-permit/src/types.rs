use candid::{CandidType, Deserialize};
use ic_stable_structures::storable::{Bound, Storable};
use icrc_ledger_types::icrc1::account::Account;
use std::borrow::Cow;

#[derive(Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StorableAccount(pub Account);

impl Storable for StorableAccount {
    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(&self.0).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(candid::decode_one(bytes.as_ref()).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, CandidType, Deserialize)]
pub struct TransferArgs {
    pub from: Account,
    pub to: Account,
    pub amount: u128,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct Permit {
    pub owner: Account,
    pub spender: Account,
    pub value: u128,
    pub deadline: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub enum TxResult {
    Ok,
    Err(String),
}
