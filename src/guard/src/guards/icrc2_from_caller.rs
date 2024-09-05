use super::{PaymentError, PaymentGuard};
use ic_cdk::api::call::{msg_cycles_accept, msg_cycles_available};
use candid::Principal;

/// The information required to deduct an ICRC-2 payment from the caller.
pub struct IcRc2FromCaller {
    caller: Principal,
    ledger_canister_id: Principal,
}

impl PaymentGuard for IcRc2FromCaller {
    fn deduct(fee: u64) -> Result<(), PaymentError> {
        let available = msg_cycles_available();
        if available < fee {
            return Err(PaymentError::InsufficientFunds {
                needed: fee,
                available,
            });
        }
        msg_cycles_accept(fee);
        Ok(())
    }
}
