use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentError {
    InsufficientFunds { needed: u64, available: u64 },
}

#[non_exhaustive]
pub enum PaymentType {
    AttachedCycles,
}

/// The account that will pay for the charge.  Compatible with the Account struct used in ICRC-2.
#[derive(CandidType, Deserialize, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}
