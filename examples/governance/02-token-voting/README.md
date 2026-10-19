# Token Voting

A minimal token-balance contract for governance examples. An administrator can
mint voting tokens, and holders can transfer them. Transfers reject insufficient
balances and recipient balance overflow without changing stored balances.

Run the tests with:

```sh
cargo test -p token-voting
```
