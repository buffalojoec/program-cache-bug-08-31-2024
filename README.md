# Cache Bug in 1.18

Program cache throws error on `assign_program` for Loader v4 and ZK Token Proof
when program-test's `warp_to_slot` is used.

## Tracing

### 1.17

In 1.17, the error does not occur. You can observe this by running the
following.

```
cd 1-17
solana-install init 1.17.3
RUST_LOG=error cargo test-sbf
```

The programs are feature-gated in 1.17.3 and do not cause any issue in the test
environment's runtime.

https://github.com/solana-labs/solana/blob/c9e8f9c81a6818db762ec15a8d22d81300c0de71/runtime/src/builtins.rs#L98-L109

### 1.18

In 1.18, the error arises. You can observe this by running the following.

```
cd 1-18
solana-install init 1.18.23
RUST_LOG=error cargo test-sbf
```

Error:

```
[2024-08-31T08:49:48.189336000Z ERROR solana_program_runtime::loaded_programs] ProgramCache::assign_program() failed key=ZkTokenProof1111111111111111111111111111111 existing=[LoadedProgram { program: LoadedProgramType::Builtin, account_size: 22, deployment_slot: 0, effective_slot: 0, maybe_expiration_slot: None, tx_usage_counter: 0, ix_usage_counter: 0, latest_access_slot: 0 }] entry=LoadedProgram { program: LoadedProgramType::Builtin, account_size: 22, deployment_slot: 0, effective_slot: 0, maybe_expiration_slot: None, tx_usage_counter: 0, ix_usage_counter: 0, latest_access_slot: 0 }
[2024-08-31T08:49:48.189527000Z ERROR solana_program_runtime::loaded_programs] ProgramCache::assign_program() failed key=LoaderV411111111111111111111111111111111111 existing=[LoadedProgram { program: LoadedProgramType::Builtin, account_size: 9, deployment_slot: 0, effective_slot: 0, maybe_expiration_slot: None, tx_usage_counter: 0, ix_usage_counter: 0, latest_access_slot: 0 }] entry=LoadedProgram { program: LoadedProgramType::Builtin, account_size: 9, deployment_slot: 0, effective_slot: 0, maybe_expiration_slot: None, tx_usage_counter: 0, ix_usage_counter: 0, latest_access_slot: 0 }
```

### 2.0

In 2.0, the error is gone again. You can observe this by running the following.

```
cd 2-0
agave-install init 2.0.8
RUST_LOG=error cargo test-sbf
```

## The Bug

The issue stems from this block that was added to 2.0, which allows builtins to
be replaced by builtins.

https://github.com/anza-xyz/agave/blob/3e7563cdad47a731b4243c3cf59a69c5fe607f07/program-runtime/src/loaded_programs.rs#L896-L904

This block is not present in 1.18. Without it, the feature gates are treated as
idempotent, which causes the runtime to try to enable the programs each time a
new bank is created.

Since the block was added to 2.0, the feature gates do in fact become
idempotent in 2.0, since 2.0's cache allows for builtin -> builtin replacement.

1.17 does not error because it does a simple `replenish`, and does not have any
of the added complexity/safeguards that were added in 1.18.

https://github.com/anza-xyz/agave/blob/77daab497df191ef485a7ad36ed291c1874596e5/program-runtime/src/loaded_programs.rs#L706

https://github.com/anza-xyz/agave/blob/77daab497df191ef485a7ad36ed291c1874596e5/program-runtime/src/loaded_programs.rs#L574-L580

Therefore, we _cannot_ activate either the Loader v4 or ZK Token Proof program
feature gates until 2.0.
