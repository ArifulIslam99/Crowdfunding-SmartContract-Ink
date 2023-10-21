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
    }

    impl Crowdfunding {
        #[ink(constructor, payable)]
        pub fn default(minimum: Balance) -> Self {
            let manager = Self::env().caller();
            let minimum_contribution = minimum;
            let approvers = Mapping::default();
            Self {
                manager,
                minimum_contribution,
                approvers,
            }
        }

        #[ink(message, payable)]
        pub fn contribute(&mut self) -> Result<(), PSP22Error> {
            let amount = Self::env().transferred_value();
            let caller = self.env().caller();
            if amount < self.minimum_contribution {
                return Err(PSP22Error::InsufficientBalance);
            } else {
                self.approvers.insert(caller, &true);
            }
            Ok(())
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
