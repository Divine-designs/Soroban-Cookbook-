#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenVotingError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InsufficientBalance = 5,
    Overflow = 6,
}

#[contract]
pub struct TokenVoting;

#[contractimpl]
impl TokenVoting {
    pub fn initialize(env: Env, admin: Address) -> Result<(), TokenVotingError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(TokenVotingError::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenVotingError> {
        require_positive(amount)?;
        let admin = read_admin(&env)?;
        admin.require_auth();

        let new_balance = read_balance(&env, &to)
            .checked_add(amount)
            .ok_or(TokenVotingError::Overflow)?;
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &new_balance);
        Ok(())
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), TokenVotingError> {
        require_positive(amount)?;
        from.require_auth();

        let from_balance = read_balance(&env, &from);
        let to_balance = read_balance(&env, &to);
        let new_from_balance = from_balance
            .checked_sub(amount)
            .filter(|balance| *balance >= 0)
            .ok_or(TokenVotingError::InsufficientBalance)?;

        if from == to {
            return Ok(());
        }

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or(TokenVotingError::Overflow)?;

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &new_from_balance);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &new_to_balance);
        Ok(())
    }

    pub fn balance(env: Env, owner: Address) -> i128 {
        read_balance(&env, &owner)
    }

    pub fn admin(env: Env) -> Result<Address, TokenVotingError> {
        read_admin(&env)
    }
}

fn require_positive(amount: i128) -> Result<(), TokenVotingError> {
    if amount <= 0 {
        Err(TokenVotingError::InvalidAmount)
    } else {
        Ok(())
    }
}

fn read_admin(env: &Env) -> Result<Address, TokenVotingError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(TokenVotingError::NotInitialized)
}

fn read_balance(env: &Env, owner: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Balance(owner.clone()))
        .unwrap_or(0)
}

#[cfg(test)]
mod test;
