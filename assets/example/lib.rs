#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod runtime_call {
    use assets_ink_sdk::{
        AssetsCall,
        RuntimeCall
    };

    #[ink(storage)]
    #[derive(Default)]
    pub struct RuntimeCaller;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RuntimeError {
        CallRuntimeFailed,
    }

    impl RuntimeCaller {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn transfer_asset(
            &mut self,
            id: u128,
            to: AccountId,
            amount: Balance,
        ) -> Result<(), RuntimeError> {
            self.env()
                .call_runtime(&RuntimeCall::Assets(AssetsCall::Transfer {
                    id,
                    target: to.into(),
                    amount
                }))
                .map_err(|_| RuntimeError::CallRuntimeFailed)
        }

        #[ink(message)]
        pub fn transfer_approve_asset(
            &mut self,
            id: u128,
            amount: Balance,
        ) -> Result<(), RuntimeError> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            self.env()
                .call_runtime(&RuntimeCall::Assets(AssetsCall::TransferApproved {
                    id,
                    owner: caller.into(),
                    destination: contract.into(),
                    amount,
                }))
                .map_err(|_| RuntimeError::CallRuntimeFailed)
        }
    }
}
