#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp22_wrapper {
    use assets_ink_sdk::{
        AssetsCall,
        RuntimeCall
    };
    use ink::codegen::{EmitEvent, Env};
    use openbrush::{contracts::psp22::*, traits::Storage};

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl psp22::Internal for PSP22WrapperContract {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
        }
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22WrapperContract {
        #[storage_field]
        psp22: psp22::Data,
        asset_id: u128,
    }

    impl PSP22 for PSP22WrapperContract {}

    impl PSP22WrapperContract {
        #[ink(constructor)]
        pub fn new(asset_id: u128) -> Self {
            let mut instance = Self::default();
            instance.asset_id = asset_id;
            instance
        }

        #[ink(message)]
        pub fn asset_id(&self) -> u128 {
            self.asset_id
        }

        /// Caller should approve contract address as spender beforehand
        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            self.env()
                .call_runtime(&RuntimeCall::Assets(AssetsCall::TransferApproved {
                    id: self.asset_id,
                    owner: caller.into(),
                    destination: contract.into(),
                    amount,
                }))
                .map_err(|_| PSP22Error::Custom("transfer failed".into()))?;
            self._mint_to(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            self._burn_from(caller, amount)?;
            self.env()
                .call_runtime(&RuntimeCall::Assets(AssetsCall::Transfer {
                    id: self.asset_id,
                    target: caller.into(),
                    amount
                }))
                .map_err(|_| PSP22Error::Custom("transfer failed".into()))
        }
    }
}