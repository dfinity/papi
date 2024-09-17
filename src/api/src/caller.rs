//! Types used primarily by the caller of the payment API.
use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;

/// How a caller states that they will pay.
#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentType {
    /// The caller is paying with cycles attached to the call.
    ///
    /// Note: This is not available for ingress messages.
    ///
    /// Note: The API does not require additional arguments to support this payment type.
    AttachedCycles,
    /// The caller is paying with cycles from their main account on the cycles ledger.
    CallerIcrc2Cycles,
    /// A patron is paying, on behalf of the caller, from their main account on the cycles ledger.
    PatronIcrc2Cycles(Principal),
}
