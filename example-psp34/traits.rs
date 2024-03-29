use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};

use crate::data::Id;
use crate::errors::{OwnableError, PSP34Error};

#[ink::trait_definition]
pub trait PSP34 {
    #[ink(message)]
    fn collection_id(&self) -> Id;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;
}

#[ink::trait_definition]
pub trait PSP34Metadata {
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}

#[ink::trait_definition]
pub trait PSP34Mintable {
    #[ink(message)]
    fn mint(&mut self, id: Id) -> Result<(), PSP34Error>;
}

#[ink::trait_definition]
pub trait PSP34Burnable {
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}

#[ink::trait_definition]
pub trait PSP34Enumerable {
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error>;

    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error>;
}

#[ink::trait_definition]
pub trait Ownable {
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;

    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;

    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError>;
}

#[ink::trait_definition]
pub trait PSP22Pausable {
    #[ink(message)]
    fn pause(&mut self) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn unpause(&mut self) -> Result<(), PSP34Error>;
}
