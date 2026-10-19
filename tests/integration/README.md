# Soroban Integration Tests - Shared Test Utilities

This directory contains comprehensive integration tests for Soroban smart contracts, along with a reusable library of test utilities and helpers.

## Table of Contents

- [Overview](#overview)
- [Test Helpers](#test-helpers)
- [Mock Contracts](#mock-contracts)
- [Test Data Generators](#test-data-generators)
- [Custom Assertions](#custom-assertions)
- [Running Tests](#running-tests)
- [Best Practices](#best-practices)
- [Migration Guide](#migration-guide)

## Overview

The test utilities library (`src/lib.rs`) provides reusable components to simplify Soroban integration testing:

- **Common Helpers**: Contract deployment, invocation, environment setup
- **Mock Contracts**: Reusable mock token and auth contracts
- **Test Data Generators**: Factories and fixtures for common test data
- **Custom Assertions**: Macros and functions for contract-specific assertions

## Test Helpers

### Basic Setup

Initialize a test environment with default Soroban configuration:

```rust
use integration_test_utils::helpers::*;

#[test]
fn example_test() {
    let env = init_test_env();
    env.mock_all_auths();
    
    // Your test code here
}
```

### Contract Deployment

Deploy a contract to the test environment:

```rust
let hello_id = deploy_contract(&env, hello_world::HelloContract);
```

### Contract Invocation

Invoke contract functions with simplified syntax:

```rust
use soroban_sdk::{symbol_short, IntoVal, Vec};

let greeting: Vec<Symbol> = invoke_contract(
    &env,
    &hello_id,
    &symbol_short!("hello"),
    &Vec::from_array(&env, [symbol_short!("Alice").into_val(&env)])
);
```

### Ledger Management

Jump forward in ledger sequence for time-dependent testing:

```rust
// Advance by 100 ledgers
jump_ledger_sequence(&env, 100);
```

## Mock Contracts

### Mock Token Contract

Create a configurable mock token for testing token interactions:

```rust
use integration_test_utils::mocks::MockTokenBuilder;

let admin = Address::generate(&env);
let token_config = MockTokenBuilder::new(admin)
    .with_supply(1_000_000)
    .with_decimals(7)
    .build();
```

### Mock Auth Contract

Create a mock authorization contract:

```rust
use integration_test_utils::mocks::MockAuthBuilder;

let admin = Address::generate(&env);
let auth_config = MockAuthBuilder::new(admin)
    .with_authorized(user1.clone())
    .with_authorized(user2.clone())
    .build();
```

## Test Data Generators

### Factory Functions

Generate common test data types:

```rust
use integration_test_utils::generators::factories::*;

let addr = address(&env, 42);           // Deterministic address from seed
let sym = symbol("count");              // Symbol from string
let val = int128(-100);                 // Signed integer
let uval = uint128(1_000_000);          // Unsigned integer
let time = timepoint(1609459200);       // Unix timestamp
```

### Token Fixture

Create pre-configured token fixtures:

```rust
use integration_test_utils::generators::fixtures::*;

let admin = Address::generate(&env);
let token = TokenFixtureBuilder::new(admin)
    .with_supply(5_000_000)
    .with_decimals(6)
    .build();
```

### Auth Fixture

Create pre-configured auth fixtures:

```rust
use integration_test_utils::generators::fixtures::*;

let admin = Address::generate(&env);
let auth = AuthFixtureBuilder::new(admin)
    .with_authorized(user1)
    .with_authorized(user2)
    .build();
```

## Custom Assertions

### Contract Success/Error Assertions

```rust
use integration_test_utils::assert_contract_success;
use integration_test_utils::assert_contract_error;

let result: Vec<Symbol> = env.invoke_contract(...);
assert_contract_success!(&result);

let error_result: Result<(), Error> = env.invoke_contract(...);
assert_contract_error!(&error_result);
```

### State Value Assertions

```rust
use integration_test_utils::helpers::assertions::assert_state_value;

assert_state_value(actual_balance, expected_balance, "balance");
```

### Collection Length Assertions

```rust
use integration_test_utils::helpers::assertions::assert_collection_length;

assert_collection_length(&events, 2, "event log");
```

### Conditional Assertions

```rust
use integration_test_utils::assert_condition;

assert_condition!(value > 0, "value should be positive, got: {}", value);
```

## Running Tests

Run all integration tests:

```bash
cargo test -p integration-tests
```

Run a specific test:

```bash
cargo test -p integration-tests test_greeting_system_workflow
```

Run tests with output:

```bash
cargo test -p integration-tests -- --nocapture
```

## Best Practices

### 1. Use `init_test_env()` for Setup

Always initialize test environment using the provided helper:

```rust
let env = init_test_env();
```

This ensures consistent environment configuration across all tests.

### 2. Use Fixtures for Complex Scenarios

For tests involving multiple contracts or data structures, use fixtures:

```rust
let admin = Address::generate(&env);
let token = TokenFixtureBuilder::new(admin)
    .with_supply(1_000_000)
    .build();
```

### 3. Use Custom Assertions for Clarity

Instead of bare `assert_eq!`, use domain-specific assertions:

```rust
// Better
assert_state_value(actual, expected, "balance");

// vs.
assert_eq!(actual, expected);
```

### 4. Group Related Tests

Use module organization to group related integration tests:

```rust
mod token_tests {
    #[test]
    fn test_transfer() { }
    
    #[test]
    fn test_balance() { }
}
```

### 5. Keep Test Data Consistent

Use factories for consistent test data generation:

```rust
let addr1 = address(&env, 1);
let addr2 = address(&env, 2);
```

## Migration Guide

### Before: Manual Contract Setup

```rust
#[test]
fn test_old_style() {
    let env = Env::default();
    env.mock_all_auths();
    
    let auth_id = env.register_contract(None, authentication::AuthContract);
    let storage_id = env.register_contract(None, storage_patterns::StorageContract);
    
    let admin = Address::generate(&env);
    
    env.invoke_contract::<()>(
        &auth_id,
        &Symbol::new(&env, "initialize"),
        Vec::from_array(&env, [admin.clone().into_val(&env)]),
    );
}
```

### After: Using Helpers

```rust
use integration_test_utils::helpers::*;

#[test]
fn test_new_style() {
    let env = init_test_env();
    
    let auth_id = deploy_contract(&env, authentication::AuthContract);
    let storage_id = deploy_contract(&env, storage_patterns::StorageContract);
    
    let admin = Address::generate(&env);
    
    invoke_contract(
        &env,
        &auth_id,
        &Symbol::new(&env, "initialize"),
        &Vec::from_array(&env, [admin.clone().into_val(&env)]),
    );
}
```

### Key Improvements

- Reduced boilerplate with `init_test_env()`
- Simplified contract deployment with `deploy_contract()`
- Cleaner invocation syntax with `invoke_contract()`

## API Reference

### Helpers Module

| Function | Purpose |
|----------|---------|
| `init_test_env()` | Initialize test environment |
| `deploy_contract()` | Deploy a contract to the environment |
| `invoke_contract()` | Invoke a contract function |
| `jump_ledger_sequence()` | Advance ledger by N ledgers |
| `setup_ledger_with_data()` | Initialize ledger state |

### Generators Module

| Function | Purpose |
|----------|---------|
| `address(seed)` | Generate deterministic address |
| `symbol(s)` | Create symbol from string |
| `int128(val)` | Create signed integer |
| `uint128(val)` | Create unsigned integer |
| `u64_val(val)` | Create u64 value |
| `u32_val(val)` | Create u32 value |

### Assertion Module

| Macro/Function | Purpose |
|---|---|
| `assert_contract_success!()` | Assert successful contract execution |
| `assert_contract_error!()` | Assert contract error |
| `assert_state_value()` | Assert state field value |
| `assert_collection_length()` | Assert collection size |
| `assert_condition!()` | Assert boolean condition |

## Contributing

When adding new tests:

1. Use the provided helpers and fixtures
2. Include documentation with examples
3. Ensure all tests pass with `cargo test -p integration-tests`
4. Run clippy: `cargo clippy -p integration-tests`
5. Update this README if adding new utilities
