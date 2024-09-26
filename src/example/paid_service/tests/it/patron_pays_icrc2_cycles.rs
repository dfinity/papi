//! Tests for the `PaymentType::PatronPaysIcrc2Cycles` payment type.
use crate::util::cycles_ledger::{Account, ApproveArgs};
use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::{CallerPaysWithIcrc2CyclesTestSetup, PaidMethods, LEDGER_FEE};
use candid::Nat;
use ic_papi_api::{principal2account, PaymentError, PaymentType};

/// Verifies that `user` can pay cycles for `user2`:
///
/// - The patron needs to approve the API cost plus the ledger fee.
/// - An unauthorized user should not be able to use that approval.
/// - `user2` should be able to make the API call.
#[test]
fn user_pays_for_user2() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    let mut expected_user_balance = CallerPaysWithIcrc2CyclesTestSetup::USER_INITIAL_BALANCE;

    // Here the user pays for user2.
    let patron = setup.user;
    let caller = setup.user2;
    let method = PaidMethods::Cost1b;
    let payment_arg = PaymentType::PatronPaysIcrc2Cycles(ic_papi_api::Account {
        owner: setup.user,
        subaccount: None,
    });

    // Pre-approve payments
    {
        setup
            .ledger
            .icrc_2_approve(
                patron,
                &ApproveArgs {
                    spender: Account {
                        owner: setup.paid_service.canister_id(),
                        subaccount: Some(principal2account(&caller)),
                    },
                    amount: Nat::from(method.cost() + LEDGER_FEE),
                    ..ApproveArgs::default()
                },
            )
            .expect("Failed to call the ledger to approve")
            .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
        // Check that the user has been charged for the approve.
        expected_user_balance -= LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user==patron balance to be charged for the ICRC2 approve".to_string(),
        );
    }
    // An unauthorized user should not be able to make a call.
    {
        let response: Result<String, PaymentError> =
            setup.call_paid_service(setup.unauthorized_user, method, &payment_arg);
        assert_eq!(
            response,
            Err(PaymentError::LedgerWithdrawFromError {
                ledger: setup.ledger.canister_id(),
                error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                    allowance: Nat::from(0u32),
                }
            }),
            "Unapproved users should not be able to make calls",
        );
        setup.assert_user_balance_eq(
            expected_user_balance,
            "The user==patron should not have been charged for unauthorized spending attempts"
                .to_string(),
        );
    }
    // user2 should be able to make the call.
    {
        // Check the canister cycle balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        let response: Result<String, PaymentError> =
            setup.call_paid_service(caller, method, &payment_arg);
        assert_eq!(
            response,
            Ok("Yes, you paid 1 billion cycles!".to_string()),
            "Should have succeeded for caller {} with patron {}.",
            caller.to_string(),
            patron.to_string(),
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
            "Expected the user==patron balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
    }
}

/// Verifies that the `PaymentType::PatronPaysIcrc2Tokens` payment type works as expected.
///
/// Here `user` is a patron, and pays on behalf of `users[2..5]`.
///
/// Only funded users should be able to make calls, and they should be able to make only as many calls as personally approved for them.
#[test]
fn user_pays_for_other_users() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    let mut expected_user_balance = CallerPaysWithIcrc2CyclesTestSetup::USER_INITIAL_BALANCE;

    // Ok, now we should be able to make an API call with EITHER an ICRC-2 approve or attached cycles, by declaring the payment type.
    // In this test, we will exercise the ICRC-2 approve.
    let api_method = "cost_1b";
    let payment_arg = PaymentType::PatronPaysIcrc2Cycles(ic_papi_api::Account {
        owner: setup.user,
        subaccount: None,
    });
    let api_fee = 1_000_000_000u128;
    let repetitions = 3;
    // Pre-approve payments
    for caller in setup.users.iter() {
        setup
            .ledger
            .icrc_2_approve(
                setup.user,
                &ApproveArgs {
                    spender: Account {
                        owner: setup.paid_service.canister_id(),
                        subaccount: Some(principal2account(caller)),
                    },
                    amount: Nat::from((api_fee + LEDGER_FEE) * repetitions),
                    ..ApproveArgs::default()
                },
            )
            .expect("Failed to call the ledger to approve")
            .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
        // Check that the user has been charged for the approve.
        expected_user_balance -= LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be charged for the ICRC2 approve".to_string(),
        );
    }
    // An unauthorized user should not be able to make a call.
    {
        let response: Result<String, PaymentError> = setup
            .paid_service
            .update(setup.unauthorized_user, api_method, &payment_arg)
            .expect("Failed to call the paid service");
        assert_eq!(
            response,
            Err(PaymentError::LedgerWithdrawFromError {
                ledger: setup.ledger.canister_id(),
                error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                    allowance: Nat::from(0u32),
                }
            }),
            "Unapproved users should not be able to make calls",
        );
        setup.assert_user_balance_eq(
            expected_user_balance,
            "The user should not have been charged for unauthorized spending attempts".to_string(),
        );
    }
    // Approved users should be able to make several API calls, up to the budget.
    let active_users = &setup.users[2..5];
    for repetition in 0..repetitions {
        // Check the balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        for caller in active_users.iter() {
            let response: Result<String, PaymentError> = setup
                .paid_service
                .update(*caller, api_method, &payment_arg)
                .expect("Failed to call the paid service");
            assert_eq!(
                response,
                Ok("Yes, you paid 1 billion cycles!".to_string()),
                "Should have succeeded for user {} on repetition {repetition}",
                caller.to_string(),
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
    // Also, additional calls by approved users, beyond the funded amount, should fail, even though there are funds left from inactive users.
    for caller in active_users.iter() {
        let response: Result<String, PaymentError> = setup
            .paid_service
            .update(*caller, api_method, &payment_arg)
            .expect("Failed to call the paid service");
        assert_eq!(
            response,
            Err(PaymentError::LedgerWithdrawFromError {
                ledger: setup.ledger.canister_id(),
                error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                    allowance: Nat::from(0u32),
                }
            }),
            "Should not be able to exceed the budget",
        );
        setup.assert_user_balance_eq(
            expected_user_balance,
            "The user should not have been charged for additional spending attempts".to_string(),
        );
    }
}
