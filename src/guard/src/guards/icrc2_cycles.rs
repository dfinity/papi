use super::{PaymentError, PaymentGuard};
use candid::{Nat, Principal};
use cycles_ledger_client::WithdrawFromArgs;
use ic_papi_api::Account;

pub struct Icrc2CyclesPaymentGuard {
    /// The payer
    pub payer_account: Account,
    /// The spender, if different from the payer.
    pub spender_subaccount: Option<serde_bytes::ByteBuf>,
    /// The ICRC-2 time, if applicable.
    pub created_at_time: Option<u64>,
    /// The ledger to withdraw the cycles from.
    pub ledger_canister_id: Principal,
    /// Own canister ID
    pub own_canister_id: Principal,
}
impl Icrc2CyclesPaymentGuard {
    pub fn default_account() -> Account {
        Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        }
    }
    pub fn default_cycles_ledger() -> Principal {
        Principal::from_text(
            option_env!("DFX_CYCLES_LEDGER_CANISTER_ID").unwrap_or("um5iw-rqaaa-aaaaq-qaaba-cai"),
        )
        .expect("Failed to parse cycles ledger canister ID")
    }
}

impl Default for Icrc2CyclesPaymentGuard {
    fn default() -> Self {
        Self {
            payer_account: Self::default_account(),
            ledger_canister_id: Self::default_cycles_ledger(),
            own_canister_id: ic_cdk::api::id(),
            created_at_time: None,
            spender_subaccount: None,
        }
    }
}

impl PaymentGuard for Icrc2CyclesPaymentGuard {
    async fn deduct(&self, fee: u64) -> Result<(), PaymentError> {
        cycles_ledger_client::Service(self.ledger_canister_id)
            .withdraw_from(&WithdrawFromArgs {
                to: self.own_canister_id,
                amount: Nat::from(fee),
                from: self.payer_account.clone(),
                spender_subaccount: self.spender_subaccount.clone(),
                created_at_time: self.created_at_time,
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
