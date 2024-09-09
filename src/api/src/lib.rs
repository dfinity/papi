use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;
use cycles_ledger_client::WithdrawFromError;

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentError {
    LedgerUnreachable {
        ledger: Principal,
    },
    LedgerError {
        ledger: Principal,
        error: WithdrawFromError,
    },
    InsufficientFunds {
        needed: u64,
        available: u64,
    },
}

#[non_exhaustive]
pub enum PaymentType {
    AttachedCycles,
}
