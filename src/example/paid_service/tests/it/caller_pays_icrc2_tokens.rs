//! Tests for the `PaymentType::CallerPaysIcrc2Tokens` payment type.
use crate::util::cycles_ledger::Account;
use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::{PaidMethods, TestSetup, LEDGER_FEE};
use candid::Nat;
use ic_papi_api::caller::CallerPaysIcrc2Tokens;
use ic_papi_api::cycles::cycles_ledger_canister_id;
use ic_papi_api::{PaymentError, PaymentType};

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected
/// on an API method that has only the corresponding guard.
///
/// Notes:
/// - The caller does not need to specify any payment arguments.  (See `call_paid_service(...)` in the test.)
/// - The caller needs to pay the API cost plus one ledger fee, for the privilege of using this payment type. (See `user_approves_payment_for_paid_service(...)` in the test.)
///   - The ledger fee may vary depending on the ledger.  The customer will need to make an allowance for the fee, either by finding out the exact fee or making an allowance with the maximum the caller is prepared to pay.
/// - This test use the cycles ledger as an ICRC-compliant ledger.
///   - TODO: Test with other ICRC-2 ledgers as well.
#[test]
fn caller_pays_icrc2_tokens() {
    let setup = TestSetup::default();
    let mut expected_user_balance = TestSetup::USER_INITIAL_BALANCE;
    // Ok, now we should be able to make an API call with an ICRC-2 approve.
    let method = PaidMethods::CallerPays1bIcrc2Tokens;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(method.cost() + LEDGER_FEE);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make an API call
    // Call the API
    let response: Result<String, PaymentError> = setup.call_paid_service(setup.user, method, ());
    assert_eq!(
        response,
        Ok("Yes, you paid 1 billion tokens!".to_string()),
        "Should have succeeded with an accurate prepayment",
    );
    expected_user_balance -= method.cost() + LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be the initial balance minus the ledger and API fees"
            .to_string(),
    );

    // The service ledger account should have been credited
    {
        let service_balance = setup
            .ledger
            .icrc_1_balance_of(
                setup.paid_service.canister_id(),
                &Account {
                    owner: setup.paid_service.canister_id(),
                    subaccount: None,
                },
            )
            .expect("Could not get service balance");
        assert_eq!(
            service_balance,
            Nat::from(method.cost()),
            "Expected the service balance to be the cost of the API call"
        );
    }
}

/// Verifies that the caller can pay for an API call with ICRC-2 tokens explicitly.
#[test]
fn caller_pays_icrc2_tokens_explicitly() {
    let setup = TestSetup::default();
    let mut expected_user_balance = TestSetup::USER_INITIAL_BALANCE;
    // Ok, now we should be able to make an API call with an ICRC-2 approve.
    let method = PaidMethods::Cost1b;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(method.cost() + LEDGER_FEE);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make an API call
    {
        let response: Result<String, PaymentError> = setup.call_paid_service(
            setup.user,
            method,
            PaymentType::CallerPaysIcrc2Tokens(CallerPaysIcrc2Tokens {
                ledger: cycles_ledger_canister_id(),
            }),
        );
        assert_eq!(
            response,
            Ok("Yes, you paid 1 billion cycles!".to_string()),
            "Should have succeeded with an accurate prepayment",
        );
    }
    // Verifies that the user account has been debited.
    {
        expected_user_balance -= method.cost() + LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
    }

    // Verifies that the canister ledger account has been credited with the payment.
    {
        let service_balance = setup
            .ledger
            .icrc_1_balance_of(
                setup.paid_service.canister_id(),
                &Account {
                    owner: setup.paid_service.canister_id(),
                    subaccount: None,
                },
            )
            .expect("Could not get service balance");
        assert_eq!(
            service_balance,
            Nat::from(method.cost()),
            "Expected the service balance to be the cost of the API call"
        );
    }
}
