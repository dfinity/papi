use super::{PaymentError, PaymentGuard};
use candid::{Nat, Principal};
use cycles_ledger_client::WithdrawFromArgs;
use ic_papi_api::{Account, Icrc2Payer};

pub struct Icrc2CyclesPaymentGuard {
    /// The payer
    pub payer: Icrc2Payer,
    /// The ledger to withdraw the cycles from.
    pub ledger_canister_id: Principal,
    /// Own canister ID
    pub own_canister_id: Principal,
}

impl PaymentGuard for Icrc2CyclesPaymentGuard {
    async fn deduct(&self, fee: u64) -> Result<(), PaymentError> {
        let account = self.payer.account.clone().unwrap_or_else(|| Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        });
        cycles_ledger_client::Service(self.ledger_canister_id)
            .withdraw_from(&WithdrawFromArgs {
                to: self.own_canister_id.clone(),
                amount: Nat::from(fee),
                from: account,
                spender_subaccount: self.payer.spender_subaccount.clone(),
                created_at_time: self.payer.created_at_time,
            })
            .await
            .map_err(|(rejection_code, string)| {
                eprintln!(
                    "Failed to reach ledger canister at {}: {rejection_code:?}: {string}",
                    self.ledger_canister_id
                );
                PaymentError::LedgerUnreachable {
                    ledger: self.ledger_canister_id,
                }
            })?
            .0
            .map_err(|e| {
                eprintln!(
                    "Failed to withdraw from ledger canister at {}: {e:?}",
                    self.ledger_canister_id
                );
                match e {
                    error => PaymentError::LedgerError {
                        ledger: self.ledger_canister_id,
                        error,
                    },
                }
            })
            .map(|_| ())
    }
}
