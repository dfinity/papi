//! Guards for specific flows

use ic_papi_api::{caller::TokenAmount, PaymentError, PaymentType};
pub mod any;
pub mod attached_cycles;
pub mod caller_pays_icrc2_cycles;
pub mod caller_pays_icrc2_tokens;
pub mod patron_pays_icrc2_cycles;
pub mod patron_pays_icrc2_tokens;

#[allow(async_fn_in_trait)]
pub trait PaymentGuard {
    async fn deduct(&self, fee: TokenAmount) -> Result<(), PaymentError>;
}

#[allow(async_fn_in_trait)]
pub trait PaymentGuard2 {
    async fn deduct(&self, payment: PaymentType, fee: TokenAmount) -> Result<(), PaymentError>;
}
