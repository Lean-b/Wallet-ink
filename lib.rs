#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod wallet {
    use ink::storage::Mapping;

 
    #[ink(storage)]
    pub struct Wallet {
        pub owner: AccountId,
        pub balances: Mapping<AccountId, Balance>,
    }


    #[ink(event)]
    pub struct Withdraw {
        to: AccountId,
        value: Balance,    
    }

    #[ink(event)]
    pub struct Deposit {
        from: AccountId,
        value: Balance,
    }

    #[derive(Debug,PartialEq, Eq, scale::Encode , scale::Decode)]  
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Wallet {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let balances = Mapping::default();
            Self {
                owner,
                balances,
            }
        }

 
        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<()> {
            let value: Balance  = self.env().transferred_value();
           
            if value == 0 {
                return Err(Error::InsufficientBalance)
            }
            
            let caller = self.env().caller();
            let balances = self.balances.get(&caller).unwrap();

            self.balances.insert(caller, &(balances + value));

            self.env().emit_event( Deposit {
                from: caller,
                value,
            });
           
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
        pub fn balance(&self) -> Option<Balance> {
            let caller = self.env().caller();
            self.balances.get(caller)
        }
        
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }
}
