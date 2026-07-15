/// Errors returned by the bridge canister.
use std::fmt;

#[derive(Debug)]
pub enum BridgeError {
    /// Candid encoding/decoding failed.
    Candid(String),
    /// Target canister rejected the proxied call.
    TargetRejected(String),
    /// Fee deduction failed (insufficient cycles/allowance/etc.).
    GuardError(String),
    /// No operator-configured price exists for the requested `(target, method)`.
    MethodNotConfigured { target: String, method: String },
    /// The configured method forwards cycles, but the chosen payment type is not
    /// cycle-denominated, so the forwarded cycles would come out of the wrapper's
    /// own balance rather than being funded by the payment.
    ForwardRequiresCyclePayment,
    /// The requested target canister may not be reached through the bridge.
    ForbiddenTarget(String),
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::Candid(e) => write!(f, "Candid error: {e}"),
            BridgeError::TargetRejected(e) => write!(f, "Target canister rejected call: {e}"),
            BridgeError::GuardError(e) => write!(f, "Payment guard error: {e}"),
            BridgeError::MethodNotConfigured { target, method } => write!(
                f,
                "No price is configured for method `{method}` on canister `{target}`. \
                 The wrapper operator must register it first."
            ),
            BridgeError::ForwardRequiresCyclePayment => write!(
                f,
                "This method forwards cycles, which requires a cycle-denominated payment type \
                 (AttachedCycles, CallerPaysIcrc2Cycles or PatronPaysIcrc2Cycles); \
                 token payments do not credit the wrapper's cycle balance."
            ),
            BridgeError::ForbiddenTarget(e) => write!(f, "Forbidden target: {e}"),
        }
    }
}

impl From<candid::Error> for BridgeError {
    fn from(e: candid::Error) -> Self {
        BridgeError::Candid(e.to_string())
    }
}
