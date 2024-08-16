//! Guards for specific flows
pub mod attached_cycles;

pub trait PaymentGuard {
    fn deduct(fee: u64) -> Result<(), PaymentError>;
}

pub enum PaymentError {
    InsufficientFunds,
}
