use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;
use cycles_ledger_client::WithdrawFromError;

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentError {
    UnsupportedPaymentType,
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

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentType {
    AttachedCycles,
    Icrc2Cycles(Icrc2Payer),
}

/// User's payment details for an ICRC2 payment.
#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct Icrc2Payer {
    /// The customer's principal and (optionally) subaccount.
    ///
    /// By default, the caller's main account is used.
    pub account: Option<Account>,
    /// The spender, if different from the payer.
    pub spender_subaccount: Option<serde_bytes::ByteBuf>,
    /// The ledger canister ID.
    ///
    /// Note: This is included in order to improve error messages if the caller tries to use the wrong ledger.
    pub ledger_canister_id: Option<Principal>,
    /// Corresponds to the `created_at_time` field in ICRC2.
    pub created_at_time: Option<u64>,
}
