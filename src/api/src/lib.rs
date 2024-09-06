use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;

#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentError {
    LedgerUnreachable { ledger: Principal },
    InsufficientFunds { needed: u64, available: u64 },
}

#[non_exhaustive]
pub enum PaymentType {
    AttachedCycles,
}
