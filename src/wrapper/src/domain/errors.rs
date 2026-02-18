/// Errors returned by the bridge canister.
///
/// Kept small because the canister is stateless; pricing/governance are pushed to the caller.
use std::fmt;

/// Errors returned by the bridge canister.
#[derive(Debug)]
pub enum BridgeError {
    /// Candid encoding/decoding failed.
    Candid(String),
    /// Target canister rejected the proxied call.
    TargetRejected(String),
    /// Fee deduction failed (insufficient cycles/allowance/etc.).
    GuardError(String),
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::Candid(e) => write!(f, "Candid error: {e}"),
            BridgeError::TargetRejected(e) => write!(f, "Target canister rejected call: {e}"),
            BridgeError::GuardError(e) => write!(f, "Payment guard error: {e}"),
        }
    }
}

impl From<candid::Error> for BridgeError {
    fn from(e: candid::Error) -> Self {
        BridgeError::Candid(e.to_string())
    }
}
