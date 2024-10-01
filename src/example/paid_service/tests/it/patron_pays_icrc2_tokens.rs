//! Tests for the `PaymentType::PatronPaysIcrc2Tokens` payment type.
use crate::util::cycles_ledger::{Account, ApproveArgs};
use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::{CallerPaysWithIcrc2CyclesTestSetup, PaidMethods, LEDGER_FEE};
use candid::Nat;
use ic_papi_api::caller::PatronPaysIcrc2Tokens;
use ic_papi_api::{principal2account, PaymentError, PaymentType};

/// Verifies that `user` can pay tokens for `user2`:
///
/// - The patron needs to approve the API cost plus the ledger fee.
/// - An unauthorized user should not be able to use that approval.
/// - `user2` should be able to make the API call.
#[test]
fn user_pays_tokens_for_user2() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    let mut expected_user_balance = CallerPaysWithIcrc2CyclesTestSetup::USER_INITIAL_BALANCE;

    // Here the user pays for user2.
    let patron = setup.user;
    let caller = setup.user2;
    let method = PaidMethods::Cost1b;
    let payment_arg = PaymentType::PatronPaysIcrc2Tokens(PatronPaysIcrc2Tokens {
        ledger: setup.ledger.canister_id(),
        patron: ic_papi_api::Account {
            owner: setup.user,
            subaccount: None,
        },
    });

    // Authorize payment
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
    // `unauthorized_user` has not paid so should not be able to make a call.
    {
        let response: Result<String, PaymentError> =
            setup.call_paid_service(setup.unauthorized_user, method, &payment_arg);
        assert_eq!(
            response,
            Err(PaymentError::LedgerTransferFromError {
                ledger: setup.ledger.canister_id(),
                error: cycles_ledger_client::TransferFromError::InsufficientAllowance {
                    allowance: Nat::from(0u32),
                }
            }),
            "Users sho have not paid should not be able to make calls",
        );
        setup.assert_user_balance_eq(
            expected_user_balance,
            "The user==patron should not have been charged for unauthorized spending attempts"
                .to_string(),
        );
    }
    // `user2` should be able to make the call.
    {
        // Call the API
        {
            let response: Result<String, PaymentError> =
                setup.call_paid_service(caller, method, &payment_arg);
            assert_eq!(
                response,
                Ok("Yes, you paid 1 billion cycles!".to_string()),
                "Should have succeeded for caller {} with patron {}.",
                caller.to_string(),
                patron.to_string(),
            );
        }
        // The patron's account should have been debited.
        {
            expected_user_balance -= method.cost() + LEDGER_FEE;
            setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user==patron balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
        }
        // The canister's ledger account should have been credited.
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
}

/// If the caller can set the vendor as patron, the caller may potentially succeed in getting free goods.
#[test]
fn user_cannot_specify_vendor_as_patron() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();

    // Here the caller will try to specify the vendor as the patron.
    let caller = setup.user;
    let patron = setup.paid_service.canister_id();
    let method = PaidMethods::Cost1b;
    let payment_arg = PaymentType::PatronPaysIcrc2Tokens(PatronPaysIcrc2Tokens {
        ledger: setup.ledger.canister_id(),
        patron: ic_papi_api::Account {
            owner: patron,
            subaccount: None,
        },
    });
    // The call should fail:
    {
        let response: Result<String, PaymentError> =
            setup.call_paid_service(caller, method, &payment_arg);
        assert_eq!(
            response,
            Err(PaymentError::InvalidPatron),
            "The caller should not be able to specify the vendor as patron.",
        );
    }
}
