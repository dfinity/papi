//! Guards for specific flows
pub mod attached_cycles;

pub trait PaymentGuard {
    fn deduct(fee: u64) -> Result<(), PaymentError>;
}

#[non_exhaustive]
pub enum PaymentError {
    InsufficientFunds,
}

#[non_exhaustive]
pub enum PaymentType {
    AttachedCycles,
}