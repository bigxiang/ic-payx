mod icrc2;
mod icrc30;
mod types;

use ic_cdk::{query, update};
use icrc_ledger_types::icrc1::account::Account;

use icrc2::{allowance, balance_of, mint, transfer};
use icrc30::verify_and_transfer;
use types::{Permit, TransferArgs, TxResult};

/// Mint new tokens to an account
#[update]
fn mint_tokens(to: Account, amount: u128) {
    mint(to, amount);
}

/// Execute a direct token transfer (ICRC-2)
#[update]
fn transfer_tokens(args: TransferArgs) -> TxResult {
    transfer(args)
}

/// Execute a permit-based transfer (ICRC-30)
/// The transfer will only succeed if the provided signature is valid.
#[update]
fn permit_transfer(p: Permit, to: Account) -> TxResult {
    verify_and_transfer(p, to)
}

/// Query the token balance of an account
#[query]
fn get_balance(owner: Account) -> u128 {
    balance_of(&owner)
}

/// Query the approved allowance from one account to another
#[query]
fn get_allowance(owner: Account, spender: Account) -> u128 {
    allowance(&owner, &spender)
}
