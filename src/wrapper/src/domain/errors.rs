/// Errors returned by the bridge canister.
///
/// Kept small because the canister is stateless; pricing/governance are pushed to the caller.
use std::fmt;

#[derive(Debug)]
pub enum BridgeError {
    /// Candid encoding/decoding failed.
    Candid(String),
    /// Target canister rejected the proxied call.
    TargetRejected(String),
    /// Fee deduction failed (insufficient cycles/allowance/etc.).
    GuardError(String),
    /// Cycles were requested to be forwarded, but the chosen payment type is not
    /// cycle-denominated, so the forwarded cycles would come out of the wrapper's
    /// own balance rather than being funded by the payment.
    ForwardRequiresCyclePayment,
    /// The requested cycles to forward exceed the fee charged, so the wrapper
    /// would be forwarding more cycles than it was paid.
    ForwardExceedsFee { fee: u128, forward: u128 },
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::Candid(e) => write!(f, "Candid error: {e}"),
            BridgeError::TargetRejected(e) => write!(f, "Target canister rejected call: {e}"),
            BridgeError::GuardError(e) => write!(f, "Payment guard error: {e}"),
            BridgeError::ForwardRequiresCyclePayment => write!(
                f,
                "Forwarding cycles requires a cycle-denominated payment type \
                 (AttachedCycles, CallerPaysIcrc2Cycles or PatronPaysIcrc2Cycles); \
                 token payments do not credit the wrapper's cycle balance."
            ),
            BridgeError::ForwardExceedsFee { fee, forward } => write!(
                f,
                "Cycles to forward ({forward}) exceed the fee charged ({fee}). \
                 The fee must cover the forwarded cycles."
            ),
        }
    }
}

impl From<candid::Error> for BridgeError {
    fn from(e: candid::Error) -> Self {
        BridgeError::Candid(e.to_string())
    }
}
