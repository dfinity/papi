//! Code to receive cycles as payment, credited to the canister, using ICRC-2 and a cycles-ledger specific withdrawal method.
use super::{PaymentError, PaymentGuardTrait};
use candid::Nat;
use ic_cycles_ledger_client::WithdrawFromArgs;
use ic_papi_api::{
    caller::TokenAmount, cycles::cycles_ledger_canister_id, principal2account, Account,
};

/// Accepts cycles using an ICRC-2 approve followed by withdrawing the cycles to the current canister.  Withdrawing
/// cycles to the current canister is specific to the cycles ledger canister; it is not part of the ICRC-2 standard.
pub struct PatronPaysIcrc2CyclesPaymentGuard {
    /// The patron paying on behalf of the caller.
    pub patron: Account,
}

impl PaymentGuardTrait for PatronPaysIcrc2CyclesPaymentGuard {
    async fn deduct(&self, fee: TokenAmount) -> Result<(), PaymentError> {
        let own_canister_id = ic_cdk::api::canister_self();
        let caller = ic_cdk::api::msg_caller();
        let spender_subaccount = Some(principal2account(&caller));
        // The patron must not be the vendor itself (this canister).
        if self.patron.owner == own_canister_id {
            return Err(PaymentError::InvalidPatron);
        }
        // The cycles ledger has a special `withdraw_from` method, similar to `transfer_from`,
        // but that adds the cycles to the canister rather than putting it into a ledger account.
        ic_cycles_ledger_client::Service(cycles_ledger_canister_id())
            .withdraw_from(&WithdrawFromArgs {
                to: own_canister_id,
                amount: Nat::from(fee),
                from: self.patron.clone(),
                spender_subaccount,
                created_at_time: None,
            })
            .await
            .map_err(|err| {
                eprintln!(
                    "Failed to reach ledger canister at {}: {err:?}",
                    cycles_ledger_canister_id()
                );
                PaymentError::LedgerUnreachable {
                    ledger: cycles_ledger_canister_id(),
                }
            })?
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
