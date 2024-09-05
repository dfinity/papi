use super::{PaymentError, PaymentGuard};
use candid::Principal;

/// The information required to deduct an ICRC-2 payment from the caller.
pub struct IcRc2FromCaller {
    caller: Principal,
    ledger_canister_id: Principal,
}

impl PaymentGuard for IcRc2FromCaller {
    fn deduct(&self, _fee: u64) -> Result<(), PaymentError> {
        unimplemented!()
    }
}
