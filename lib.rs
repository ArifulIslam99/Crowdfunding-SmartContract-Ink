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
        requests: Mapping<RequestId, Request>,
        request_id: RequestId,
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
 
    pub struct Request {
        request_id: RequestId,
        description: Vec<u8>,
        value: Balance,
        recipient: AccountId,
        complete: bool,
        voters: Vec<AccountId>,
        approval_count: u32
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
                requests,
                request_id: 0,
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
        pub fn create_spending_request(
            &mut self,
            description: Vec<u8>,
            value: Balance,
            recipient: AccountId,
        ) -> Result<RequestId, PSP22Error> {
            self.manager_call();
            let new_request = Request {
                request_id: self.request_id,
                description,
                value,
                recipient,
                complete: false,
                voters: Vec::new(),
                approval_count: 0
            };

            self.requests.insert(self.request_id, &new_request);
            self.increment_request_id();
            Ok(self.request_id - 1)
        }

        #[ink(message)]
        pub fn get_manger_address(&self) -> AccountId {
            self.manager
        }

        fn manager_call(&self) {
            assert_eq!(
                self.manager,
                self.env().caller(),
                "Only owner can call this function"
            );
        }

        pub fn increment_request_id(&mut self) -> RequestId {
            self.request_id += 1;
            self.request_id
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
