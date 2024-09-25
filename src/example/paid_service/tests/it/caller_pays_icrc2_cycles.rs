use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::{CallerPaysWithIcrc2CyclesTestSetup, PaidMethods, LEDGER_FEE};
use candid::Nat;
use ic_papi_api::PaymentError;

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works providing that the
/// ICRC2 approval is sufficient.
#[test]
fn caller_pays_icrc2_cycles() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Exercise the protocol...
    let api_fee = 1_000_000_000u128;
    for payment in (api_fee - 5)..(api_fee + 5) {
        for _repetition in 0..2 {
            // Pre-approve payment
            setup.user_approves_payment_for_paid_service(payment + LEDGER_FEE);
            // Check that the user has been charged for the approve.
            expected_user_balance -= LEDGER_FEE;
            setup.assert_user_balance_eq(
                expected_user_balance,
                "Expected the user balance to be charged for the ICRC2 approve".to_string(),
            );

            // Check the balance beforehand
            let service_canister_cycles_before =
                setup.pic.cycle_balance(setup.paid_service.canister_id);
            // Call the API
            let response: Result<String, PaymentError> =
                setup.call_paid_service(setup.user, PaidMethods::Cost1bIcrc2Cycles);
            if payment < api_fee {
                assert_eq!(
                    response,
                    Err(PaymentError::LedgerError {
                        ledger: setup.ledger.canister_id(),
                        error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                            allowance: Nat::from(payment + LEDGER_FEE),
                        }
                    }),
                    "Should have failed with only {} cycles attached",
                    payment
                );
                setup.assert_user_balance_eq(
                    expected_user_balance,
                    "Expected the user balance to be unchanged by a failed ICRC2".to_string(),
                );
            } else {
                assert_eq!(
                    response,
                    Ok("Yes, you paid 1 billion cycles!".to_string()),
                    "Should have succeeded with {} cycles attached",
                    payment
                );
                let service_canister_cycles_after =
                    setup.pic.cycle_balance(setup.paid_service.canister_id);
                assert!(
                    service_canister_cycles_after > service_canister_cycles_before,
                    "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
                    service_canister_cycles_before - service_canister_cycles_after
                );
                expected_user_balance -= api_fee + LEDGER_FEE;
                setup.assert_user_balance_eq(
                expected_user_balance,
                "Expected the user balance to be the initial balance minus the ledger and API fees"
                    .to_string(),
            );
            }
        }
    }
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected:
///
/// - The user's main cycles account has cycles deducted.
/// - The cycle balance of the canister providing the paid service increases.
///   - Note: Given that the canister consumes cycles as part of the operation, we check that the balance increases but do not check an exact amount.
///
/// Note: The method used is: cost_1b_icrc2_from_caller

#[test]
fn caller_pays_icrc2_cycles_with_payment_arg_works() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Exercise the protocol...
    let api_fee = 1_000_000_000u128;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(expected_user_balance);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make several identical API calls
    for _repetition in 0..5 {
        // Check the balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        let response: Result<String, PaymentError> =
            setup.call_paid_service(setup.user, PaidMethods::Cost1bIcrc2Cycles);
        assert_eq!(
            response,
            Ok("Yes, you paid 1 billion cycles!".to_string()),
            "Should have succeeded with a generous prepayment",
        );
        let service_canister_cycles_after = setup.pic.cycle_balance(setup.paid_service.canister_id);
        assert!(
            service_canister_cycles_after > service_canister_cycles_before,
            "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
            service_canister_cycles_before - service_canister_cycles_after
        );
        expected_user_balance -= api_fee + LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
    }
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected
/// on an API method that takes a payment argument.
///
/// Note: The method used is: `cost_1b``
#[test]
fn caller_pays_by_named_icrc2() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Ok, now we should be able to make an API call with an ICRC-2 approve.
    let api_fee = 1_000_000_000u128;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(expected_user_balance);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make several identical API calls
    for _repetition in 0..5 {
        // Check the balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        let response: Result<String, PaymentError> =
            setup.call_paid_service(setup.user, PaidMethods::Cost1b);
        assert_eq!(
            response,
            Ok("Yes, you paid 1 billion cycles!".to_string()),
            "Should have succeeded with a generous prepayment",
        );
        let service_canister_cycles_after = setup.pic.cycle_balance(setup.paid_service.canister_id);
        assert!(
            service_canister_cycles_after > service_canister_cycles_before,
            "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
            service_canister_cycles_before - service_canister_cycles_after
        );
        expected_user_balance -= api_fee + LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
        // But an unauthorized user should not be able to make the same call.
        {
            let response: Result<String, PaymentError> =
                setup.call_paid_service(setup.unauthorized_user, PaidMethods::Cost1b);
            assert_eq!(
                response,
                Err(PaymentError::LedgerError {
                    ledger: setup.ledger.canister_id(),
                    error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                        allowance: Nat::from(0u32),
                    }
                }),
                "A user who hasn't paid should not be able to make the call",
            );
            setup.assert_user_balance_eq(
                expected_user_balance,
                "The user should not have been charged for unauthorized spending attempts"
                    .to_string(),
            );
        }
    }
}
