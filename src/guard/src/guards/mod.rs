//! Guards for specific flows

use ic_papi_api::PaymentError;
pub mod attached_cycles;
pub mod icrc2_cycles;
pub mod any;

#[allow(async_fn_in_trait)]
pub trait PaymentGuard {
    async fn deduct(&self, fee: u64) -> Result<(), PaymentError>;
}
