//! Code to receive any ICRC-2 token as payment.

// Well known ICRC-2 tokens
// TODO

use super::{PaymentError, PaymentGuard};
use candid::{Nat, Principal};
use cycles_ledger_client::TransferFromArgs;
use ic_papi_api::{caller::TokenAmount, cycles::cycles_ledger_canister_id, Account};

pub struct CallerPaysIcrc2TokensPaymentGuard {
    /// The ledger for that specific token
    pub ledger: Principal,
}

impl PaymentGuard for CallerPaysIcrc2TokensPaymentGuard {
    async fn deduct(&self, cost: TokenAmount) -> Result<(), PaymentError> {
        let caller = ic_cdk::api::caller();
        cycles_ledger_client::Service(cycles_ledger_canister_id())
            .icrc_2_transfer_from(&TransferFromArgs {
                from: Account {
                    owner: caller,
                    subaccount: None,
                },
                to: Account {
                    owner: ic_cdk::api::id(),
                    subaccount: None,
                },
                amount: Nat::from(cost),
                spender_subaccount: None,
                created_at_time: None,
                memo: None,
                fee: None,
            })
            .await
            .map_err(|(rejection_code, string)| {
                eprintln!(
                    "Failed to reach ledger canister at {}: {rejection_code:?}: {string}",
                    cycles_ledger_canister_id()
                );
                PaymentError::LedgerUnreachable {
                    ledger: cycles_ledger_canister_id(),
                }
            })?
            .0
            .map_err(|error| {
                eprintln!(
                    "Failed to withdraw from ledger canister at {}: {error:?}",
                    cycles_ledger_canister_id()
                );
                PaymentError::LedgerTransferFromError {
                    ledger: cycles_ledger_canister_id(),
                    error,
                }
            })
            .map(|_| ())
    }
}

/*

pub struct Icrc2TokensPaymentGuard {
    /// The ledger for that specific token.
    pub ledger: Principal,
    /// The payer; either the caller or a patron.
    pub payer_account: Account,
    /// The vendor's subaccount for the caller, if the payment is by a patron.
    pub spender_subaccount: Option<serde_bytes::ByteBuf>,
    /// Own canister ID
    pub own_canister_id: Principal,
}

impl PaymentGuard for Icrc2TokensPaymentGuard {
    async fn deduct(&self, cost: TokenAmount) -> Result<(), PaymentError> {
        cycles_ledger_client::Service(cycles_ledger_canister_id())
            .icrc_2_transfer_from(&TransferFromArgs {
                from: self.payer_account.clone(),
                to: Account {
                    owner: self.own_canister_id,
                    subaccount: None,
                },
                amount: Nat::from(cost),
                spender_subaccount: self.spender_subaccount.clone(),
                created_at_time: None,
                memo: None,
                fee: None,
            })
            .await
            .map_err(|(rejection_code, string)| {
                eprintln!(
                    "Failed to reach ledger canister at {}: {rejection_code:?}: {string}",
                    cycles_ledger_canister_id()
                );
                PaymentError::LedgerUnreachable {
                    ledger: cycles_ledger_canister_id(),
                }
            })?
            .0
            .map_err(|error| {
                eprintln!(
                    "Failed to withdraw from ledger canister at {}: {error:?}",
                    cycles_ledger_canister_id()
                );
                PaymentError::LedgerTransferFromError {
                    ledger: cycles_ledger_canister_id(),
                    error,
                }
            })
            .map(|_| ())
    }
}
*/
