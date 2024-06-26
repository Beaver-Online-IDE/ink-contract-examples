
use crate::PSP22Error;
use ink::prelude::string::String;
use ink::{
    prelude::{vec, vec::Vec},
    primitives::AccountId,
    storage::Mapping,
};
use ink::env::call::{build_call, ExecutionInput, Selector};
use ink::env::DefaultEnvironment;

pub enum PSP22Event {
    Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: u128,
    },
    Approval {
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    },
}

#[ink::storage_item]
#[derive(Debug, Default)]
pub struct PSP22Data {
    total_supply: u128,
    balances: Mapping<AccountId, u128>,
    allowances: Mapping<(AccountId, AccountId), u128>,
}

impl PSP22Data {
    pub fn new(supply: u128, creator: AccountId) -> PSP22Data {
        let mut data = PSP22Data {
            total_supply: supply,
            balances: Default::default(),
            allowances: Default::default(),
        };
        data.balances.insert(creator, &supply);
        data
    }

    pub fn total_supply(&self) -> u128 {
        self.total_supply
    }

    pub fn balance_of(&self, owner: AccountId) -> u128 {
        self.balances.get(owner).unwrap_or_default()
    }

    pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
        self.allowances.get((owner, spender)).unwrap_or_default()
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        value: u128,
    ) -> Result<Vec<PSP22Event>, PSP22Error> {
        if caller == to || value == 0 {
            return Ok(vec![]);
        }
        let from_balance = self.balance_of(caller);
        if from_balance < value {
            return Err(PSP22Error::InsufficientBalance);
        }

        if from_balance == value {
            self.balances.remove(caller);
        } else {
            self.balances
                .insert(caller, &(from_balance.saturating_sub(value)));
        }
        let to_balance = self.balance_of(to);
        self.balances
            .insert(to, &(to_balance.saturating_add(value)));
        Ok(vec![PSP22Event::Transfer {
            from: Some(caller),
            to: Some(to),
            value,
        }])
    }

    pub fn transfer_from(
        &mut self,
        caller: AccountId,
        from: AccountId,
        to: AccountId,
        value: u128,
    ) -> Result<Vec<PSP22Event>, PSP22Error> {
        if from == to || value == 0 {
            return Ok(vec![]);
        }
        if caller == from {
            return self.transfer(caller, to, value);
        }

        let allowance = self.allowance(from, caller);
        if allowance < value {
            return Err(PSP22Error::InsufficientAllowance);
        }
        let from_balance = self.balance_of(from);
        if from_balance < value {
            return Err(PSP22Error::InsufficientBalance);
        }

        if allowance == value {
            self.allowances.remove((from, caller));
        } else {
            self.allowances
                .insert((from, caller), &(allowance.saturating_sub(value)));
        }

        if from_balance == value {
            self.balances.remove(from);
        } else {
            self.balances
                .insert(from, &(from_balance.saturating_sub(value)));
        }
        let to_balance = self.balance_of(to);
        self.balances
            .insert(to, &(to_balance.saturating_add(value)));
        Ok(vec![
            PSP22Event::Approval {
                owner: from,
                spender: caller,
                amount: allowance.saturating_sub(value),
            },
            PSP22Event::Transfer {
                from: Some(from),
                to: Some(to),
                value,
            },
        ])
    }

    pub fn approve(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: u128,
    ) -> Result<Vec<PSP22Event>, PSP22Error> {
        if owner == spender {
            return Ok(vec![]);
        }
        if value == 0 {
            self.allowances.remove((owner, spender));
        } else {
            self.allowances.insert((owner, spender), &value);
        }
        Ok(vec![PSP22Event::Approval {
            owner,
            spender,
            amount: value,
        }])
    }

    pub fn increase_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        delta_value: u128,
    ) -> Result<Vec<PSP22Event>, PSP22Error> {
        if owner == spender || delta_value == 0 {
            return Ok(vec![]);
        }
        let allowance = self.allowance(owner, spender);
        let amount = allowance.saturating_add(delta_value);
        self.allowances.insert((owner, spender), &amount);
        Ok(vec![PSP22Event::Approval {
            owner,
            spender,
            amount,
        }])
    }

    pub fn decrease_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        delta_value: u128,
    ) -> Result<Vec<PSP22Event>, PSP22Error> {
        if owner == spender || delta_value == 0 {
            return Ok(vec![]);
        }
        let allowance = self.allowance(owner, spender);
        if allowance < delta_value {
            return Err(PSP22Error::InsufficientAllowance);
        }
        let amount = allowance.saturating_sub(delta_value);
        if amount == 0 {
            self.allowances.remove((owner, spender));
        } else {
            self.allowances.insert((owner, spender), &amount);
        }
        Ok(vec![PSP22Event::Approval {
            owner,
            spender,
            amount,
        }])
    }

    pub fn mint(&mut self, to: AccountId, value: u128) -> Result<Vec<PSP22Event>, PSP22Error> {
        if value == 0 {
            return Ok(vec![]);
        }
        let new_supply = self
            .total_supply
            .checked_add(value)
            .ok_or(PSP22Error::Custom(String::from(
                "Max PSP22 supply exceeded. Max supply limited to 2^128-1.",
            )))?;
        self.total_supply = new_supply;
        let new_balance = self.balance_of(to).saturating_add(value);
        self.balances.insert(to, &new_balance);
        Ok(vec![PSP22Event::Transfer {
            from: None,
            to: Some(to),
            value,
        }])
    }

    pub fn burn(&mut self, from: AccountId, value: u128) -> Result<Vec<PSP22Event>, PSP22Error> {
        if value == 0 {
            return Ok(vec![]);
        }

        let balance = self.balance_of(from);
        if balance < value {
            return Err(PSP22Error::InsufficientBalance);
        }
        if balance == value {
            self.balances.remove(from);
        } else {
            self.balances.insert(from, &(balance.saturating_sub(value)));
        }
        self.total_supply = self.total_supply.saturating_sub(value);
        Ok(vec![PSP22Event::Transfer {
            from: Some(from),
            to: None,
            value,
        }])
    }

    pub fn burn_from(&mut self,
                     caller: AccountId,
                     from: AccountId,
                     value: u128
    ) -> Result<Vec<PSP22Event>, PSP22Error> {
        if value == 0 {
            return Ok(vec![]);
        }
        let allowance = self.allowance(from, caller);
        if allowance < value {
            return Err(PSP22Error::InsufficientAllowance);
        }
        let balance = self.balance_of(from);
        if balance < value {
            return Err(PSP22Error::InsufficientBalance);
        }

        if allowance == value {
            self.allowances.remove((from, caller));
        } else {
            self.allowances
                .insert((from, caller), &(allowance.saturating_sub(value)));
        }
        if balance == value {
            self.balances.remove(from);
        } else {
            self.balances.insert(from, &(balance.saturating_sub(value)));
        }
        self.total_supply = self.total_supply.saturating_sub(value);
        Ok(vec![PSP22Event::Transfer {
            from: Some(from),
            to: None,
            value,
        }])
    }

    pub fn deposit(&mut self,
                   underlying: AccountId,
                   sender: AccountId,
                   contract: AccountId,
                   value: u128
    ) -> Result<(), PSP22Error> {
        pub const TRANSFER_FROM_SELECTOR: [u8; 4] = [84, 179, 199, 110];

        build_call::<DefaultEnvironment>()
            .call(underlying)
            .gas_limit(0)
            .transferred_value(0)
            .exec_input(
                ExecutionInput::new(Selector::new(TRANSFER_FROM_SELECTOR))
                    .push_arg(sender)
                    .push_arg(contract)
                    .push_arg(value)
                    .push_arg(Vec::<u8>::new())
            )
            .returns::<Result<(), PSP22Error>>()
            .invoke()
    }

    pub fn withdraw(&mut self,
                    underlying: AccountId,
                    account: AccountId,
                    value: u128
    ) -> Result<(), PSP22Error> {
        pub const TRANSFER_SELECTOR: [u8; 4] = [219, 32, 249, 245];

        build_call::<DefaultEnvironment>()
            .call(underlying)
            .gas_limit(0)
            .transferred_value(0)
            .exec_input(
                ExecutionInput::new(Selector::new(TRANSFER_SELECTOR))
                    .push_arg(account)
                    .push_arg(value)
                    .push_arg(Vec::<u8>::new())
            )
            .returns::<Result<(), PSP22Error>>()
            .invoke()
    }
}