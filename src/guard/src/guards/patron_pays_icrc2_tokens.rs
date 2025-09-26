//! Code to receive cycles as payment, credited to the canister, using ICRC-2 and a cycles-ledger specific withdrawal method.
use super::{PaymentError, PaymentGuardTrait};
use candid::{Nat, Principal};
use ic_cycles_ledger_client::TransferFromArgs;
use ic_papi_api::{caller::TokenAmount, principal2account, Account};

/// Accepts cycles using an ICRC-2 approve followed by withdrawing the cycles to the current canister.  Withdrawing
/// cycles to the current canister is specific to the cycles ledger canister; it is not part of the ICRC-2 standard.
pub struct PatronPaysIcrc2TokensPaymentGuard {
    /// The ledger for that specific token
    pub ledger: Principal,
    /// The patron paying on behalf of the caller
    pub patron: Account,
}

impl PaymentGuardTrait for PatronPaysIcrc2TokensPaymentGuard {
    async fn deduct(&self, cost: TokenAmount) -> Result<(), PaymentError> {
        let caller = ic_cdk::api::msg_caller();
        let own_canister_id = ic_cdk::api::canister_self();
        let spender_subaccount = principal2account(&caller);
        // The patron must not be the vendor itself (this canister).
        if self.patron.owner == own_canister_id {
            return Err(PaymentError::InvalidPatron);
        }
        // Note: The cycles ledger client is ICRC-2 compatible so can be used here.
        let result = ic_cycles_ledger_client::Service(self.ledger)
            .icrc_2_transfer_from(&TransferFromArgs {
                from: self.patron.clone(),
                to: Account {
                    owner: ic_cdk::api::canister_self(),
                    subaccount: None,
                },
                amount: Nat::from(cost),
                spender_subaccount: Some(spender_subaccount),
                created_at_time: None,
                memo: None,
                fee: None,
            })
            .await
            .map_err(|error| {
                eprintln!(
                    "Failed to reach ledger canister at {}: {error:?}",
                    self.ledger
                );
                PaymentError::LedgerUnreachable {
                    ledger: self.ledger,
                }
            })?;

        result.0
            .map_err(|error| {
                eprintln!(
                    "Failed to withdraw from ledger canister at {}: {error:?}",
                    self.ledger
                );
                PaymentError::LedgerTransferFromError {
                    ledger: self.ledger,
                    error,
                }
            })
            .map(|_| ())
    }
}
