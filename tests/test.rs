#![cfg_attr(not(feature = "std"), no_std)]

#[path = "../src/lib.rs"]
mod subscrypt;

mod utils;

#[cfg(test)]
pub mod tests {
    use crate::subscrypt::subscrypt::LinkedList;
    use crate::subscrypt::subscrypt::Subscrypt;
    use crate::utils::utils::{
        set_account_balance, set_caller, subscrypt_add_plan_scenario, subscrypt_edit_plan_scenario,
        subscrypt_provider_register_scenario,
    };
    use ink_env::hash::{HashOutput, Sha2x256};
    use ink_lang as ink;

    #[ink::test]
    fn constructor_works() {
        let subscrypt = Subscrypt::new();
        assert_eq!(subscrypt.provider_register_fee, 100);
    }

    #[ink::test]
    fn default_works() {
        let subscrypt = Subscrypt::default();
        assert_eq!(subscrypt.provider_register_fee, 100);
    }
    #[ink::test]
    fn linked_list_works() {
        let linked = LinkedList::new();
        assert_eq!(linked.back, 0);
    }

    #[ink::test]
    fn linked_list_default_works() {
        let linked = LinkedList::default();
        assert_eq!(linked.back, 0);
    }

    #[ink::test]
    fn provider_register_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
    }

    #[ink::test]
    #[should_panic]
    fn provider_register_works2() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_caller(callee, accounts.alice, 90);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
    }

    #[ink::test]
    #[should_panic]
    fn provider_register_works3() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_caller(callee, accounts.alice, 90);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
    }

    #[ink::test]
    fn edit_plan_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
        subscrypt_edit_plan_scenario(
            &mut subscrypt,
            accounts.alice,
            1,
            60 * 60 * 24 * 10,
            3,
            100000,
            500,
            false,
        );
    }

    #[ink::test]
    #[should_panic]
    fn edit_plan_works2() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
        subscrypt_edit_plan_scenario(
            &mut subscrypt,
            accounts.alice,
            2,
            60 * 60 * 24 * 10,
            3,
            100000,
            500,
            false,
        );
    }
    #[ink::test]
    fn add_plan_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
        subscrypt_add_plan_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24 * 10],
            vec![3],
            vec![100000],
            vec![500],
        )
    }

    #[ink::test]
    #[should_panic]
    fn add_plan_works2() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
        subscrypt_add_plan_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24 * 10],
            vec![3, 2],
            vec![100000],
            vec![500],
        );
    }

    #[ink::test]
    fn change_disable_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        assert_eq!(
            subscrypt
                .providers
                .get(&accounts.alice)
                .unwrap()
                .money_address,
            accounts.alice
        );
        subscrypt.change_disable(1);
        assert_eq!(
            subscrypt
                .providers
                .get(&accounts.alice)
                .unwrap()
                .plans
                .get(1)
                .unwrap()
                .disabled,
            true
        );

        subscrypt.change_disable(1);
        assert_eq!(
            subscrypt
                .providers
                .get(&accounts.alice)
                .unwrap()
                .plans
                .get(1)
                .unwrap()
                .disabled,
            false
        );
    }

    #[ink::test]
    fn subscribe_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .users
                .get(&accounts.bob)
                .unwrap()
                .list_of_providers
                .get(0)
                .unwrap(),
            &accounts.alice
        );
    }

    #[ink::test]
    #[should_panic]
    fn subscribe_works2() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
        set_caller(callee, accounts.bob, 49500);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .users
                .get(&accounts.bob)
                .unwrap()
                .list_of_providers
                .get(0)
                .unwrap(),
            &accounts.alice
        );
    }

    #[ink::test]
    fn withdraw_works() {
        let mut subscrypt = Subscrypt::new();

        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);
        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .users
                .get(&accounts.bob)
                .unwrap()
                .list_of_providers
                .get(0)
                .unwrap(),
            &accounts.alice
        );
        set_caller(callee, accounts.alice, 0);
        subscrypt.withdraw();
    }

    #[ink::test]
    #[should_panic]
    fn withdraw_works2() {
        let mut subscrypt = Subscrypt::new();

        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);
        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .users
                .get(&accounts.bob)
                .unwrap()
                .list_of_providers
                .get(0)
                .unwrap(),
            &accounts.alice
        );

        set_caller(callee, accounts.eve, 0);
        subscrypt.withdraw();
    }

    #[ink::test]
    fn refund_works() {
        let mut subscrypt = Subscrypt::new();

        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );

        subscrypt.refund(accounts.alice, 1);
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            true
        );
    }

    #[ink::test]
    #[should_panic]
    fn refund_works2() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );
        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );

        subscrypt.refund(accounts.alice, 1);
        subscrypt.refund(accounts.alice, 1);
    }

    #[ink::test]
    fn check_subscription_works() {
        let mut subscrypt = Subscrypt::new();

        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");

        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );
        assert_eq!(
            subscrypt.check_subscription(accounts.bob, accounts.alice, 1),
            true
        );
    }

    #[ink::test]
    fn retrieve_data_with_wallet_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );
        let s = subscrypt.retrieve_data_with_wallet(accounts.alice);
        assert_eq!(s[0].provider, accounts.alice);
        assert_eq!(s[0].plan_index, 1);
        assert_eq!(s[0].plan.duration, 60 * 60 * 24 * 30);
    }

    #[ink::test]
    fn retrieve_whole_data_with_wallet_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );
        let s = subscrypt.retrieve_whole_data_with_wallet();
        assert_eq!(s[0].provider, accounts.alice);
        assert_eq!(s[0].plan_index, 1);
        assert_eq!(s[0].plan.duration, 60 * 60 * 24 * 30);
    }

    #[ink::test]
    fn retrieve_data_with_password_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);
        let t: String = "token".to_string();
        let p: String = "pass_phrase".to_string();
        let encodable = [t, p];
        let mut output = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
        ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut output);

        subscrypt.subscribe(accounts.alice, 1, output, "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );
        let s = subscrypt.retrieve_data_with_password(
            accounts.bob,
            accounts.alice,
            "token".parse().unwrap(),
            "pass_phrase".parse().unwrap(),
        );
        assert_eq!(s[0].provider, accounts.alice);
        assert_eq!(s[0].plan_index, 1);
        assert_eq!(s[0].plan.duration, 60 * 60 * 24 * 30);
    }

    #[ink::test]
    fn retrieve_whole_data_with_password_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        let t: String = "token".to_string();
        let p: String = "pass_phrase".to_string();
        let encodable = [t, p];
        let mut output = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
        ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut output);

        subscrypt.subscribe(accounts.alice, 1, output, "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );
        let s = subscrypt.retrieve_whole_data_with_password(
            accounts.bob,
            "token".parse().unwrap(),
            "pass_phrase".parse().unwrap(),
        );
        assert_eq!(s[0].provider, accounts.alice);
        assert_eq!(s[0].plan_index, 1);
        assert_eq!(s[0].plan.duration, 60 * 60 * 24 * 30);
    }

    #[ink::test]
    fn check_auth_works() {
        let mut subscrypt = Subscrypt::new();
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_account_balance(callee, 50100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![60 * 60 * 24, 60 * 60 * 24 * 30],
            vec![2, 2],
            vec![10000, 50000],
            vec![50, 100],
        );

        set_caller(callee, accounts.bob, 50000);

        let t: String = "token".to_string();
        let p: String = "pass_phrase".to_string();
        let encodable = [t, p];
        let mut output = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
        ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut output);

        subscrypt.subscribe(accounts.alice, 1, output, "nothing important".to_string());
        assert_eq!(
            subscrypt
                .records
                .get(&(accounts.bob, accounts.alice))
                .unwrap()
                .subscription_records
                .get(0)
                .unwrap()
                .refunded,
            false
        );
        let s = subscrypt.check_auth(
            accounts.bob,
            accounts.alice,
            "token".parse().unwrap(),
            "pass_phrase".parse().unwrap(),
        );
        assert_eq!(s, true);
    }

    #[ink::test]
    fn add_entry_works() {
        let mut subscrypt = Subscrypt::new();

        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        let callee =
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id");
        set_account_balance(callee, 90100);
        set_caller(callee, accounts.alice, 100);
        subscrypt_provider_register_scenario(
            &mut subscrypt,
            accounts.alice,
            vec![
                60 * 60 * 24,
                60 * 60 * 24 * 30,
                60 * 60 * 24 * 300,
                60 * 60 * 24 * 31,
            ],
            vec![2, 2, 2, 2],
            vec![10000, 50000, 10000, 10000],
            vec![50, 100, 200, 100],
        );
        set_caller(callee, accounts.bob, 50000);

        subscrypt.subscribe(accounts.alice, 1, [0; 32], "nothing important".to_string());
        set_caller(callee, accounts.bob, 10000);

        subscrypt.subscribe(accounts.alice, 0, [0; 32], "nothing important".to_string());

        subscrypt.subscribe(accounts.alice, 2, [0; 32], "nothing important".to_string());
        set_caller(callee, accounts.eve, 10000);

        subscrypt.subscribe(accounts.alice, 0, [0; 32], "nothing important".to_string());
        subscrypt.subscribe(accounts.alice, 3, [0; 32], "nothing important".to_string());
        assert_eq!(
            subscrypt
                .users
                .get(&accounts.bob)
                .unwrap()
                .list_of_providers
                .get(0)
                .unwrap(),
            &accounts.alice
        );
        set_caller(callee, accounts.alice, 0);

        subscrypt.process(accounts.alice, 1000);
    }
}
