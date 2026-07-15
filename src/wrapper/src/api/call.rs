use ic_papi_api::PaymentType;

use crate::domain::errors::BridgeError;
use crate::domain::types::BridgeCallArgs;
use crate::payments::guard_config::PAYMENT_GUARD;
use crate::util::cycles::forward_raw;

fn map_guard_err<E: core::fmt::Debug>(e: E) -> String {
    BridgeError::GuardError(format!("{e:?}")).to_string()
}

/// Internal helper to unify the bridge logic: charge fee -> forward call.
// TODO: The caller may have to provide more type information than they are used to. Normally dfx will use the target canister's candid file to convert to the correct types; without that information it will guess more simply and won't always get this conversion right.
pub async fn bridge_call(args: BridgeCallArgs) -> Result<Vec<u8>, String> {
    if args.target == ic_cdk::api::canister_self() {
        return Err("Self-calls are not allowed through the bridge.".to_string());
    }

    let p = args.payment.unwrap_or(PaymentType::AttachedCycles);
    let cycles = args.cycles_to_forward.unwrap_or(0);

    // 1) Validate that the forwarded cycles are covered by the fee *before*
    //    charging anything.
    //
    //    The forwarded cycles are attached to the outbound call from this
    //    canister's own balance. They are only replenished when the payment is
    //    cycle-denominated: `AttachedCycles` (via `msg_cycles_accept`) and the
    //    ICRC-2 *cycles* flows (via the cycles-ledger `withdraw_from(to: self)`)
    //    credit this canister's cycle balance; the ICRC-2 *token* flows credit a
    //    token account instead. Since `fee_amount` and `cycles_to_forward` are
    //    both caller-controlled, we must ensure the fee is paid in cycles and
    //    covers the amount forwarded -- otherwise a caller could pay a trivial
    //    fee and drain the wrapper's cycles.
    if cycles > 0 {
        if !is_cycle_payment(&p) {
            return Err(BridgeError::ForwardRequiresCyclePayment.to_string());
        }
        if args.fee_amount < cycles {
            return Err(BridgeError::ForwardExceedsFee {
                fee: args.fee_amount,
                forward: cycles,
            }
            .to_string());
        }
    }

    // 2) Charge fee using the payment guard
    PAYMENT_GUARD
        .deduct(p, args.fee_amount)
        .await
        .map_err(map_guard_err)?;

    // 3) Forward the call to target
    forward_raw(args.target, &args.method, args.args, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}

/// Whether a payment type credits this canister's *cycle* balance (as opposed to
/// a token ledger account), and can therefore fund forwarded cycles.
fn is_cycle_payment(payment: &PaymentType) -> bool {
    matches!(
        payment,
        PaymentType::AttachedCycles
            | PaymentType::CallerPaysIcrc2Cycles
            | PaymentType::PatronPaysIcrc2Cycles(_)
    )
}
