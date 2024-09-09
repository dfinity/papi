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
    Icrc2(Icrc2Payment),
}

/// User's payment details for an ICRC2 payment.
#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct Icrc2Payment {
    /// The user's principal and (optionally) subaccount.
    ///
    /// By default, the caller's main account is used.
    pub payer: Option<Account>,
    /// The ledger canister ID.
    ///
    /// By default, the cycles ledger is used.  A given canister MAY accept other currencies.
    pub ledger_canister_id: Option<Principal>,
    /// Corresponds to the `created_at_time` field in ICRC2.
    pub created_at_time: Option<u64>,
}
