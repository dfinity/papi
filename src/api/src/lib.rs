use candid::{CandidType, Deserialize, Principal};
pub use cycles_ledger_client::Account;
pub use error::PaymentError;
use serde_bytes::ByteBuf;

pub mod error;

#[derive(Debug, CandidType, Deserialize, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentType {
    /// The caller is paying with cycles attached to the call.
    ///
    /// Note: This is not available for ingress messages.
    ///
    /// Note: The API does not require additional arguments to support this payment type.
    AttachedCycles,
    /// The caller is paying with cycles from their main account on the (by default cycles) ledger.
    CallerIcrc2,
    /// A patron is paying, on behalf of the caller, from their main account on the (by default cycles) ledger.
    PatronIcrc2(Principal),
}

pub fn principal2account(principal: &Principal) -> ByteBuf {
    // TODO: This is NOT the right way.
    let mut ans = principal.as_slice().to_vec();
    while ans.len() < 32 {
        ans.push(0);
    }
    ByteBuf::from(ans)
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
