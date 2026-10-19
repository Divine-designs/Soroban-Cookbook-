#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

struct Fixture {
    token: TokenVotingClient<'static>,
    alice: Address,
    bob: Address,
}

fn setup() -> Fixture {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(TokenVoting, ());
    let token = TokenVotingClient::new(&env, &contract_id);
    token.initialize(&admin);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    Fixture { token, alice, bob }
}

#[test]
fn transfer_moves_voting_tokens() {
    let f = setup();
    f.token.mint(&f.alice, &100);

    f.token.transfer(&f.alice, &f.bob, &40);

    assert_eq!(f.token.balance(&f.alice), 60);
    assert_eq!(f.token.balance(&f.bob), 40);
}

#[test]
fn transfer_rejects_insufficient_balance() {
    let f = setup();
    f.token.mint(&f.alice, &10);

    assert_eq!(
        f.token.try_transfer(&f.alice, &f.bob, &11),
        Err(Ok(TokenVotingError::InsufficientBalance))
    );
    assert_eq!(f.token.balance(&f.alice), 10);
    assert_eq!(f.token.balance(&f.bob), 0);
}

#[test]
fn transfer_rejects_recipient_balance_overflow() {
    let f = setup();
    f.token.mint(&f.alice, &1);
    f.token.mint(&f.bob, &i128::MAX);

    assert_eq!(
        f.token.try_transfer(&f.alice, &f.bob, &1),
        Err(Ok(TokenVotingError::Overflow))
    );
    assert_eq!(f.token.balance(&f.alice), 1);
    assert_eq!(f.token.balance(&f.bob), i128::MAX);
}

#[test]
fn mint_rejects_balance_overflow() {
    let f = setup();
    f.token.mint(&f.alice, &i128::MAX);

    assert_eq!(
        f.token.try_mint(&f.alice, &1),
        Err(Ok(TokenVotingError::Overflow))
    );
    assert_eq!(f.token.balance(&f.alice), i128::MAX);
}

#[test]
fn self_transfer_preserves_balance() {
    let f = setup();
    f.token.mint(&f.alice, &10);

    f.token.transfer(&f.alice, &f.alice, &5);

    assert_eq!(f.token.balance(&f.alice), 10);
}
