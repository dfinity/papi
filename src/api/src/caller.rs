//! Types used primarily by the caller of the payment API.
use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;

/// How a caller states that they will pay.
#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentType {
    /// The caller is paying with cycles attached to the call.
    ///
    /// Note: This is available to inter-canister aclls only; not to ingress messages.
    ///
    /// Note: The API does not require additional arguments to support this payment type.
    AttachedCycles,
    /// The caller is paying with cycles from their main account on the cycles ledger.
    CallerPaysIcrc2Cycles,
    /// A patron is paying, on behalf of the caller, from their main account on the cycles ledger.
    PatronPaysIcrc2Cycles(PatronPaysIcrc2Cycles),
    /// The caller is paying with tokens from their main account on the specified ledger.
    CallerPaysIcrc2Token(CallerPaysIcrc2Token),
    /// A patron is paying, on behalf of the caller, from their main account on the specified ledger.
    PatronPaysIcrc2Token(PatronPaysIcrc2Token),
}

pub type PatronPaysIcrc2Cycles = Principal;

#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct CallerPaysIcrc2Token {
    pub ledger: Principal,
}

#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct PatronPaysIcrc2Token {
    pub ledger: Principal,
    pub patron: Principal,
}

pub type TokenAmount = u64;
