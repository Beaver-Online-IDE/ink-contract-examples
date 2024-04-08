#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod types_sandbox {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum OwnableError {}

    #[ink::storage_item]
    pub struct Inner {
        value: bool,
    }

    #[ink(storage)]
    pub struct TypesSandbox {
        bool_value: bool,
        u8_value: u8,
        name: String,
        values_list: Vec<i128>,
        owner: Option<AccountId>,
        inner: Inner,
    }

    impl TypesSandbox {

        #[ink(message)]
        pub fn get_inner_value(&self) -> bool {
            self.inner.value
        }

        #[ink(message)]
        pub fn delete_from_list(&mut self, element_index: u64) -> Result<(), OwnableError> {
            assert_eq!(
                Some(self.env().caller()),
                self.owner,
                "Only owner can insert into list"
            );
            self.values_list.remove(element_index as usize);
            Ok(())
        }

        #[ink(message)]
        pub fn insert_into_list(&mut self, new_value: i128) {
            assert_eq!(
                Some(self.env().caller()),
                self.owner,
                "Only owner can insert into list"
            );
            self.values_list.push(new_value)
        }

        #[ink(message)]
        pub fn get_list_size(&self) -> u64 {
            self.values_list.len() as u64
        }

        #[ink(message)]
        pub fn get_u8_value(&self) -> u8 {
            self.u8_value
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn set_name(&mut self, new_name: String) {
            self.name = new_name
        }

        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                bool_value: init_value,
                u8_value: 15,
                name: String::from("initial string"),
                values_list: Vec::new(),
                owner: Some(Self::env().caller()),
                inner: Inner { value: true },
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.bool_value = !self.bool_value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.bool_value
        }
    }
}
