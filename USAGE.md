# Usage

## Creating a paid API

### Accept many payment types

- In your canister, you need to decide which payment types you wish to accept. In [this example](./src/example/paid_service/src/state.rs) a `PAYMENT_GUARD` is defined that accepts a variety of payment options.
- In the API methods, you need to [deduct payment](./src/example/paid_service/src/lib.rs) before doing work.
