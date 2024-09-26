use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::{CallerPaysWithIcrc2CyclesTestSetup, PaidMethods, LEDGER_FEE};
use candid::{Nat, Principal};
use ic_papi_api::{PaymentError, PaymentType};
use ic_papi_guard::guards::any::PaymentWithConfig;

/// Verifies that the paid API is using the expected config for the `PaymentType::CallerPaysIcrc2Cycles`.
#[test]
fn caller_pays_icrc2_cycles_payment_config() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    let response: Option<PaymentWithConfig> = setup.paid_service.update(
        Principal::anonymous(),
        "payment_config",
        PaymentType::CallerPaysIcrc2Cycles,
    ).unwrap();
    assert_eq!(
        response,
        Some(PaymentWithConfig::CallerPaysIcrc2Cycles),
        "The payment configuration for CallerPaysIcrc2Cycles should be set to the ledger with a fee of {}",
        LEDGER_FEE
    );
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected with a range of approval amounts near the required amount.
///
/// - The call should succeed if the ICRC2 approval is greater than or equal to the cost of the method.
/// - The user's main cycles account has cycles deducted when a call succeeds.
/// - The cycle balance of the canister providing the paid service increases when a call succeeds.
///  - Note: Given that the canister consumes cycles as part of the operation, we check that the balance increases but do not check an exact amount.
#[test]
fn caller_pays_icrc2_cycles_works_with_large_enough_approval() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the user's cycles ledger account.
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

    // Try calling a method with a range of approval amounts.  The call should succeed if the
    // ICRC2 approval is greater than or equal to the cost of the method.
    let method = PaidMethods::Cost1bIcrc2Cycles;
    for payment in (method.cost() - 5)..(method.cost() + 5) {
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
                setup.call_paid_service(setup.user, method, ());
            if payment < method.cost() {
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
                expected_user_balance -= method.cost() + LEDGER_FEE;
                setup.assert_user_balance_eq(
                expected_user_balance,
                "Expected the user balance to be the initial balance minus the ledger and API fees"
                    .to_string(),
            );
            }
        }
    }
}

/// Verifies that a user can pay for multiple API calls with a single approval.
#[test]
fn caller_pays_icrc2_cycles_supports_multiple_calls_with_a_single_approval() {
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
    // Pre-approve a large sum.
    setup.user_approves_payment_for_paid_service(expected_user_balance);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make several API calls
    let method = PaidMethods::Cost1bIcrc2Cycles;
    for _repetition in 0..5 {
        // Check the balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        let response: Result<String, PaymentError> = setup.call_paid_service(setup.user, method, ());
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
        expected_user_balance -= method.cost() + LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
    }
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected
/// on an API method that takes a payment argument.
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
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Ok, now we should be able to make an API call with an ICRC-2 approve.
    let method = PaidMethods::Cost1b;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(method.cost());
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make an API call
    // Check the canister cycles balance beforehand
    let service_canister_cycles_before = setup.pic.cycle_balance(setup.paid_service.canister_id);
    // Call the API
    let response: Result<String, PaymentError> =
        setup.call_paid_service(setup.user, PaidMethods::Cost1b, PaymentType::CallerPaysIcrc2Cycles);
    assert_eq!(
        response,
        Ok("Yes, you paid 1 billion cycles!".to_string()),
        // "Should have succeeded with a generous prepayment",
        "\nsetup ledger:   {:?}\nmainnet ledger: {:?}\npapi ledger:    {:?}",
        setup.ledger.canister_id(),
        Principal::from_text(ic_papi_api::cycles::MAINNET_CYCLES_LEDGER_CANISTER_ID).unwrap(),
        ic_papi_api::cycles::cycles_ledger_canister_id()
    );
    let service_canister_cycles_after = setup.pic.cycle_balance(setup.paid_service.canister_id);
    assert!(
        service_canister_cycles_after > service_canister_cycles_before,
        "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
        service_canister_cycles_before - service_canister_cycles_after
    );
    expected_user_balance -= method.cost() + LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be the initial balance minus the ledger and API fees"
            .to_string(),
    );
}
