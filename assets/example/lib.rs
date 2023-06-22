#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod runtime_call {
    use assets_ink_sdk::{
        AssetsCall,
        RuntimeCall
    };
    use ink::env::Error as EnvError;

    /// A trivial contract with a single message, that uses `call-runtime` API for
    /// performing native token transfer.
    #[ink(storage)]
    #[derive(Default)]
    pub struct RuntimeCaller;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RuntimeError {
        CallRuntimeFailed,
    }

    impl From<EnvError> for RuntimeError {
        fn from(e: EnvError) -> Self {
            match e {
                EnvError::CallRuntimeFailed => RuntimeError::CallRuntimeFailed,
                _ => panic!("Unexpected error from `pallet-contracts`."),
            }
        }
    }

    impl RuntimeCaller {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn transfer_asset(
            &mut self,
            id: u32,
            amount: Balance,
        ) -> Result<(), RuntimeError> {
            let contract = self.env().account_id();
            self.env()
                .call_runtime(&RuntimeCall::Assets(AssetsCall::Transfer {
                    id,
                    target: contract.into(),
                    amount,
                }))
                .map_err(Into::into)
        }
    }
}
