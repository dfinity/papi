use super::{PaymentError, PaymentGuard};
use candid::Principal;

/// The information required to deduct an ICRC-2 payment from the caller.
pub struct Icrc2FromCaller {
    /// The payer
    pub payer: ic_papi_api::Account,
    /// The ledger to deduct the charge from.
    pub ledger_canister_id: Principal,
}

impl PaymentGuard for Icrc2FromCaller {
    fn deduct(&self, _fee: u64) -> Result<(), PaymentError> {
        ic_cdk::api::call::call(
            self.ledger_canister_id,
            "",
            (self.payer, _fee),
        );
        unimplemented!()
    }
}
