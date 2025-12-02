
/// Errors returned by the bridge canister.
///
/// Kept small because the canister is stateless; pricing/governance are pushed to the caller.
#[derive(Debug)]
pub enum BridgeError {
    /// Candid encoding/decoding failed.
    Candid(String),
    /// Target canister rejected the proxied call.
    TargetRejected(String),
    /// Fee deduction failed (insufficient cycles/allowance/etc.).
    GuardError(String),
}

impl ToString for BridgeError {
    fn to_string(&self) -> String {
        use BridgeError::*;
        match self {
            Candid(e) => format!("Candid: {e}"),
            TargetRejected(e) => format!("TargetRejected: {e}"),
            GuardError(e) => format!("GuardError: {e}"),
        }
    }
}
