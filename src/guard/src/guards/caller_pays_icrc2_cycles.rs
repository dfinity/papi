//! Code to receive cycles as payment, credited to the canister, using ICRC-2 and a cycles-ledger specific withdrawal method.
use super::{PaymentError, PaymentGuardTrait};
use candid::Nat;
use ic_cycles_ledger_client::WithdrawFromArgs;
use ic_papi_api::{caller::TokenAmount, cycles::cycles_ledger_canister_id, Account};

/// Accepts cycles using an ICRC-2 approve followed by withdrawing the cycles to the current canister.  Withdrawing
/// cycles to the current canister is specific to the cycles ledger canister; it is not part of the ICRC-2 standard.
#[derive(Default)]
pub struct CallerPaysIcrc2CyclesPaymentGuard {}

impl PaymentGuardTrait for CallerPaysIcrc2CyclesPaymentGuard {
    async fn deduct(&self, fee: TokenAmount) -> Result<(), PaymentError> {
        let caller = ic_cdk::caller();
        let own_canister_id = ic_cdk::api::id();
        let payer_account = Account {
            owner: caller,
            subaccount: None,
        };
        // The patron must not be the vendor itself (this canister).
        if payer_account.owner == own_canister_id {
            return Err(PaymentError::InvalidPatron);
        }
        // The cycles ledger has a special `withdraw_from` method, similar to `transfer_from`,
        // but that adds the cycles to the canister rather than putting it into a ledger account.
        ic_cycles_ledger_client::Service(cycles_ledger_canister_id())
            .withdraw_from(&WithdrawFromArgs {
                to: own_canister_id,
                amount: Nat::from(fee),
                from: payer_account,
                spender_subaccount: None,
                created_at_time: None,
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
                PaymentError::LedgerWithdrawFromError {
                    ledger: cycles_ledger_canister_id(),
                    error,
                }
            })
            .map(|_| ())
    }
}
