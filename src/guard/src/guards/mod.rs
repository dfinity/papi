//! Guards for specific flows

use ic_papi_api::PaymentError;
pub mod attached_cycles;
pub mod icrc2_from_caller;

pub trait PaymentGuard {
    fn deduct(fee: u64) -> Result<(), PaymentError>;
}
