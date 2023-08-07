#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod wallet {
    use ink::{primitives::AccountId, env::{call, caller}};



    #[ink(storage)]
    pub struct Wallet {
        pub owner: AccountId,
        pub balances: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
       from: AccountId,
       to:AccountId,
       value: Balance ,
    }
    

    #[ink(event)]
    pub struct Withdraw {
        to: AccountId,
        value: Balance,    
    }

    #[ink(event)]
    pub struct Deposit {
        from: AccountId,
        value: balance,
    }

     

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    pub type Result<T> = core::result::Result<T,Error>;

    impl Wallet {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                balances: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                owner: Default::default(),              
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<()> {
            let value: Balance  = self.env().transferred_value();
           
            if value == 0 {
                return Err(Error::InsufficientBalance)
            }
            
            self.balance += value;
           
            Ok(())
        }

        
        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<()>{
            let caller = self.env().caller();
            let balance = self.balances.get(caller).unwrap();
            
            if balance == 0 {
                return Err(Error::InsufficientBalance)
            }

            self.balances.remove(caller);
            self.env().transfer(caller, balance).unwrap();
            self.env().emit_event(Withdraw{
                to: caller,
                value: amount,
            });

            Ok(())
        }

        

        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(),Error> {
            
                
            Ok(())
        }
       
       
        #[ink(message)]
        pub fn balance(&self) -> Balance {
            self.balances
        }
        
        #[ink(message)]
        pub fn get_owner(&self, owner: AccountId) -> AccountId {
            self.owner
        }
    }
}

/// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
///
/// When running these you need to make sure that you:
/// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
/// - Are running a Substrate node which contains `pallet-contracts` in the background
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    /// A helper function used for calling contract messages.
    use ink_e2e::build_message;

    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;

    /// The End-to-End test `Result` type.
    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// We test that we can upload and instantiate the contract using its default constructor.
    #[ink_e2e::test]
    async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let constructor = WalletRef::default();

        // When
        let contract_account_id = client
            .instantiate("wallet", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // Then
        let get = build_message::<WalletRef>(contract_account_id.clone())
            .call(|wallet| wallet.get());
        let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), false));

        Ok(())
    }

    /// We test that we can read and write a value from the on-chain contract contract.
    #[ink_e2e::test]
    async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let constructor = WalletRef::new(false);
        let contract_account_id = client
            .instantiate("wallet", &ink_e2e::bob(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let get = build_message::<WalletRef>(contract_account_id.clone())
            .call(|wallet| wallet.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), false));

        // When
        let flip = build_message::<WalletRef>(contract_account_id.clone())
            .call(|wallet| wallet.flip());
        let _flip_result = client
            .call(&ink_e2e::bob(), flip, 0, None)
            .await
            .expect("flip failed");

        // Then
        let get = build_message::<WalletRef>(contract_account_id.clone())
            .call(|wallet| wallet.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }
}

