#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod crowdfunding {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    
    #[ink(storage)]
    pub struct Crowdfunding {
        manager: AccountId,
        minimum_contribution: Balance,
        approvers: Mapping<AccountId, bool>,
        requests: Mapping<RequestId, request>
    }
    pub type RequestId = u32;
    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct request {
        description: Vec<u8>,
        value: Balance,
        recipient: AccountId,
        complete: bool
    }

    impl Crowdfunding {
        #[ink(constructor)]
        pub fn default(minimum: Balance) -> Self {
            let manager = Self::env().caller();
            let minimum_contribution = minimum;
            let approvers = Mapping::default();
            let requests = Mapping::default();
            Self {
                manager,
                minimum_contribution,
                approvers,
                requests
            }
        }

        #[ink(message, payable)]
        pub fn contribute(&mut self) -> Result<(), PSP22Error> {
            let amount = Self::env().transferred_value();
            let caller = self.env().caller();
            assert_ne!(self.manager, caller);
            if amount < self.minimum_contribution {
                return Err(PSP22Error::InsufficientBalance);
            } else {
                self.approvers.insert(caller, &true);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn get_manger_address(&self) -> AccountId{
            self.manager
        }

        
         fn manager_call(&self) {
            assert_eq!(
                self.manager,
                self.env().caller(),
                "Only owner can call this function"
            );
        }
    }

    //* Handling Error Efficiently  **//
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP22Error {
        InsufficientBalance, //Not Enough Balance
        Custom(Vec<u8>),     // Specofy the reason for terminating transaction
    }
}
