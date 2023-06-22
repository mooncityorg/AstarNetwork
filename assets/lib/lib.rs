#![cfg_attr(not(feature = "std"), no_std, no_main)]

use sp_runtime::MultiAddress;
use ink::primitives::AccountId;

#[derive(scale::Encode)]
pub enum RuntimeCall {
    #[codec(index = 17)]
    Assets(AssetsCall),
}

#[derive(scale::Encode)]
pub enum AssetsCall {
    #[codec(index = 8)]
    Transfer {
        #[codec(compact)]
        id: u128,
        target: MultiAddress<AccountId, ()>,
        #[codec(compact)]
        amount: u128,
    },
    #[codec(index = 25)]
    TransferApproved {
        #[codec(compact)]
        id: u128,
        owner: MultiAddress<AccountId, ()>,
        destination: MultiAddress<AccountId, ()>,
        #[codec(compact)]
        amount: u128,
    },
}