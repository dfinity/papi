[![Code checks](https://github.com/dfinity/papi/actions/workflows/check.yml/badge.svg)](https://github.com/dfinity/papi/actions/workflows/check.yml)
[![Integration checks](https://github.com/dfinity/papi/actions/workflows/integration.yml/badge.svg)](https://github.com/dfinity/papi/actions/workflows/integration.yml)

# Get paid for your API.

## TLDR

**`papi` adds a payment gateway to an API. Choose which cryptocurrencies you would like to be paid in, and how much each API call should cost, and the `papi` integration will handle the rest.**

## Details

### Choice of cryptocurrency

The following cryptocurrencies are currently supported:

| Name       | Token  | Comment                                                                                               |
| ---------- | ------ | ----------------------------------------------------------------------------------------------------- |
| Bitcoin    | ckBTC  | High speed, low transaction costs are enabled via decentralized chain keys.                           |
| Ethereum   | ckETH  | High speed, low transaction costs are enabled via decentralized chain keys.                           |
| US Dollar  | ckUSDC | High speed, low transaction costs are enabled via decentralized chain keys.                           |
| ICP Cycles | XDR    | The native utility token of the Internet Computer, tied in price to the IMF XDR basket of currencies. |
| ICP        | ICP    | The governance token of the Internet Computer.                                                        |

And many more. All tokens that support the ICRC-2 standard can be used. We are considering how best to add other currencies such as native Eth; ck\* tokens provide fast and inexpensive settlement but there will be use cases where native tokens may be wanted.

### Chain keys: ckBTC, ckETH, ckUSDC, ...

APIs require high speed, low latency and low transaction fees. Otherwise the user experience will be terrible. Chain Key provides a standard, cryptocurrency-agnostic, decentralized way of delivering these necessary properties. If you are excited by technical details, you will be glad to know that Chain Key Technology enables making L2s on the ICP with threshold keys. ICP provides the high performance and the threshold keys provide the foundation for making the L2 decentralized.

### Technical Integration

You will need to define a default currency for payment and annotate API methods with how much you would like to charge for each call. The payment method can be either passed explicitly by the caller or you can specify one fixed payment method in your canister. Payment is currently supported by attached cycles or ICRC2 transfer; more methods are likely to be added in future. For ICRC-2, the customer will have to approve the payment in advance. In the case of payment with ICP cycles, payment is attached directly to the API call.

<!-- NOT IMPLEMENTED YET
This flow can be customized by providing explicit payment parameters. For every API method you have, another will be added with the `paid_` prefix and the payment parameter. For example, if you have an API method `is_prime(x: u32) -> bool`, a method will be added `paid_is_prime(payment_details, u32) -> Result<bool, PaymentError>`. The default flow has the advantage that you do not need to alter your API in any way. With this explicit payment mechanism you have more options, such as support for multiple currencies and payment by accounts other than the caller.

Optionally, pre-payment is also supported. In this case, the `papi` library will need to store customer credits in stable memory and you will need to set the duration for which pre-paid credits are valid.
-->

#### Examples

This API requires payment in cycles, directly to the canister. The acceptable payment types are configured like this:

```rust
pub static PAYMENT_GUARD: LazyLock<PaymentGuard<3>> = LazyLock::new(|| PaymentGuard {
     supported: [
            VendorPaymentConfig::AttachedCycles,
            VendorPaymentConfig::CallerPaysIcrc2Cycles,
            VendorPaymentConfig::PatronPaysIcrc2Cycles,
        ],
});
```

The API is protected like this:

```
#[update]
is_prime(x: u32, payment: Option<PaymentType>) -> Result<bool, PaymentError> {
  let fee = 1_000_000_000;
  PAYMENT_GUARD.deduct(payment.unwrap_or(VendorPaymentConfig::AttachedCycles), fee).await?;
  // Now check whether the number really is prime:
  ...
}
```

A user MAY pay by attaching cycles directly to API call:

```
dfx canister call "$MATH_CANISTER_ID" --with-cycles 10000 is_prime '(1234567)'
```

A user MAY also pre-approve payment, then make the call:

```
dfx canister call $CYCLES_LEDGER icrc2_approve '
  record {
    amount = 10000;
    spender = record {
      owner = principal "'${MATH_CANISTER_ID}'";
    };
  }
'

dfx canister call "$MATH_CANISTER_ID" is_prime '(1234567, opt variant { CallerPaysIcrc2Cycles })'
```

Finally, there are complex use cases where another user pays on behalf of the caller. In this case, the payer needs to set aside some funds for the caller in a sub-account and approve the payment. The funds can be used only by that caller:

```
# Payer:
## Add funds to a subaccount for the caller:
CALLER_ACCOUNT="$(dfx ledger account-id --of-principal "$CALLER")"
SUBACCOUNT_ID="$(dfx ledger account-id --subaccount "$CALLER_ACCOUNT")"
dfx cycles transfer "$SUBACCOUNT_ID" 200000
## Authorize payment:
dfx canister call $CYCLES_LEDGER icrc2_approve '
  record {
    amount = 10000;
    from_subaccount = "'${CALLER_ACCOUNT}'";
    spender = record {
      owner = principal "'${MATH_CANISTER_ID}'";
    };
  }
'

# Caller:
## The caller needs to specify the payment source explicitly:
PAYER_ACCOUNT="$(dfx ledger account-id --of-principal "$PAYER")"
dfx canister call "$MATH_CANISTER_ID" paid_is_prime '
(
  1234,
  opt variant {
    PatronPaysIcrc2Cycles = record {
      owner = principal "PAYER_ACCOUNT";
    }
  },
)
'
```

Your canister will retrieve the pre-approved payment before proceeding with the API call.

## The Wrapper Canister

### What it is

The **wrapper** is a standalone ICP canister that adds payment enforcement to **any existing canister** — with zero changes to that canister's code. Instead of modifying an API canister to integrate `papi` directly, you deploy the wrapper in front of it. Callers send their requests to the wrapper, which charges the fee and then proxies the call through to the real destination.

This makes `papi` accessible even for canisters you do not own, for third-party APIs, or for teams who prefer to keep business logic and payment logic completely separate.

### How it works

The wrapper exposes a small set of generic proxy methods:

| Method      | Description                                          |
| ----------- | ---------------------------------------------------- |
| `call0`     | Proxy a call that takes **no arguments**             |
| `call_blob` | Proxy a call with a **Candid-encoded argument blob** |

Every proxy method follows the same two-step internal logic:

1. **Charge the fee** – the payment guard deducts the requested amount from the caller (or from a designated payer) using any supported payment type (attached cycles, ICRC-2 approve, patron pays, …).
2. **Forward the call** – once the fee is settled, the wrapper performs a raw inter-canister call to the target canister and method, passing the arguments and any optional cycles through transparently.

If the fee deduction fails the call is rejected immediately and the target canister is never reached. Self-calls (calling the wrapper itself) are blocked.

### Flow diagram

```mermaid
sequenceDiagram
    participant Caller
    participant Payer
    participant Wrapper as Wrapper Canister (papi)
    participant Ledger as Token Ledger (ICRC-2)
    participant Target as Destination Canister

    Note over Payer,Ledger: Optional – only needed for patron-pays or ICRC-2 flows
    Payer->>Ledger: icrc2_approve(spender=Wrapper, amount=fee)

    Caller->>Wrapper: call_blob / call0(target, method, args, fee, payment?)
    activate Wrapper
    Wrapper->>Ledger: icrc2_transfer_from (or deduct attached cycles)
    Ledger-->>Wrapper: ok
    Wrapper->>Target: raw canister call (method, args)
    Target-->>Wrapper: response bytes
    deactivate Wrapper
    Wrapper-->>Caller: Result<response, error>
```

> **Payer ≠ Caller** — in the simplest case both roles are fulfilled by the same identity. When using `PatronPaysIcrc2Cycles` the payer pre-approves a budget on behalf of the caller, so the caller never has to hold or manage funds directly.

### Usage

#### 1. Deploy or locate the wrapper canister

You can deploy the wrapper yourself from `src/wrapper`, or use the shared instance already published to the IC mainnet (see `canister_ids.json`).

#### 2. Call a payable endpoint

**Paying with attached cycles (simplest):**

```bash
dfx canister call "$WRAPPER_ID" call0 \
  '(record {
    target         = principal "'$TARGET_CANISTER_ID'";
    method         = "my_method";
    fee_amount     = 1_000_000 : nat;
    payment        = null;          # defaults to AttachedCycles
    cycles_to_forward = null;
  })' \
  --with-cycles 1000000
```

**Paying with an ICRC-2 token (e.g. ckUSDC) — caller pays:**

```bash
# 1. Approve the wrapper to spend from your account
dfx canister call $LEDGER icrc2_approve '(record {
  amount  = 5_000_000;
  spender = record { owner = principal "'$WRAPPER_ID'" };
})'

# 2. Call through the wrapper
dfx canister call "$WRAPPER_ID" call_blob '(record {
  target        = principal "'$TARGET_CANISTER_ID'";
  method        = "store_data";
  args_blob     = blob "\44\49\44\4c\00\00";
  fee_amount    = 5_000_000 : nat;
  payment       = opt variant { CallerPaysIcrc2Cycles };
  cycles_to_forward = null;
})'
```

**Patron pays on behalf of the caller:**

```bash
# Payer approves a per-caller budget
dfx canister call $LEDGER icrc2_approve '(record {
  amount           = 10_000_000;
  from_subaccount  = opt blob "'$CALLER_SUBACCOUNT'";
  spender          = record { owner = principal "'$WRAPPER_ID'" };
})'

# Caller references the payer when calling through the wrapper
dfx canister call "$WRAPPER_ID" call0 '(record {
  target        = principal "'$TARGET_CANISTER_ID'";
  method        = "my_method";
  fee_amount    = 1_000_000 : nat;
  payment       = opt variant {
    PatronPaysIcrc2Cycles = record { owner = principal "'$PAYER_ID'" }
  };
  cycles_to_forward = null;
})'
```

#### Key parameters

| Parameter           | Type              | Description                                                |
| ------------------- | ----------------- | ---------------------------------------------------------- |
| `target`            | `Principal`       | The canister to forward the call to                        |
| `method`            | `Text`            | The method name on the target canister                     |
| `fee_amount`        | `Nat`             | Fee charged by the wrapper before forwarding               |
| `payment`           | `opt PaymentType` | Payment mechanism; defaults to `AttachedCycles` if omitted |
| `args_blob`         | `Blob`            | Candid-encoded arguments (for `call_blob`)                 |
| `cycles_to_forward` | `opt Nat`         | Additional cycles to pass to the target alongside the call |

The response is returned as `Result<blob, text>`: the raw Candid-encoded response bytes on success, or an error string describing what went wrong (guard failure or target rejection).

---

## Licence & Contribution

This repository is released under the [Apache2 license](./LICENSE).

Unfortunately we are unable to accept contributions yet. When we do, we will provide a contribution guide.
