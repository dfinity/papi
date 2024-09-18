//! Types used primartily by the vendor of the payment API.
use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;

/// Billing options.
#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentOption {
    /// The caller is paying with cycles attached to the call.
    ///
    /// Note: This is available to inter-canister aclls only; not to ingress messages.
    ///
    /// Note: The API does not require additional arguments to support this payment type.
    AttachedCycles { fee: Option<u64> },
    /// The caller is paying with cycles from their main account on the cycles ledger.
    CallerPaysIcrc2Cycles { fee: Option<u64> },
    /// A patron is paying, on behalf of the caller, from their main account on the cycles ledger.
    PatronPaysIcrc2Cycles { fee: Option<u64> },
}

/// Vendor payment configuration, including details that may not necessarily be shared with the customer.
#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub enum VendorPaymentConfig {
    /// Cycles are received by the vendor canister.
    AttachedCycles,
    /// Cycles are received by the vendor canister.
    CallerPaysIcrc2Cycles,
    /// Cycles are received by the vendor canister.
    PatronPaysIcrc2Cycles,
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
