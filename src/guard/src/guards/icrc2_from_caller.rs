use super::{PaymentError, PaymentGuard};
use candid::{Nat, Principal};
use cycles_ledger_client::WithdrawFromArgs;

/// The information required to deduct an ICRC-2 payment from the caller.
pub struct Icrc2FromCaller {
    /// The payer
    pub payer: cycles_ledger_client::Account,
    /// The ledger to deduct the charge from.
    pub ledger_canister_id: Principal,
    /// Own canister ID
    pub own_canister_id: Principal,
}

impl PaymentGuard for Icrc2FromCaller {
    async fn deduct(&self, fee: u64) -> Result<(), PaymentError> {
        cycles_ledger_client::Service(self.ledger_canister_id)
            .withdraw_from(&WithdrawFromArgs {
                to: self.own_canister_id.clone(),
                from: self.payer.clone(),
                amount: Nat::from(fee),
                spender_subaccount: None,
                created_at_time: None,
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
                // TODO: Improve error handling
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
