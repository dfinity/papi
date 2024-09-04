use candid::{CandidType, Deserialize};

#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentError {
    InsufficientFunds,
}

#[non_exhaustive]
pub enum PaymentType {
    AttachedCycles,
}
