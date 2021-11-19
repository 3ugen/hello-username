### 1. Check the latest version Rust SDK and Example projects

- [Changelog](https://github.com/near/near-sdk-rs/blob/master/CHANGELOG.md)
  <br/>
- [Issues](https://github.com/near/near-sdk-rs/issues)
- What about backward compatibility ?
- [NEAR Rust SDK Getting Started](https://www.near-sdk.io/) 💯
- self learn, additional sources of truth
  <br/>google search: near site:hackmd.io/ <br/>and
  particular [NEAR Contract Basics line by line](https://hackmd.io/@nearly-learning/contract-basics-rust)
  <br/>[Youtube playlist (Rust for Near, RUS Lang)](https://www.youtube.com/playlist?list=PL9tzQn_TEuFUakOn-IY9cDQL2ztNzZunh) 
  <br/>[Github Examples from NEAR](https://github.com/near-examples)
  <br/>[Github Examples Near-SDK-Rust](https://github.com/near/near-sdk-rs/tree/master/examples)
  <br/>[Github Compilation tests Near-SDK-Rust](https://github.com/near/near-sdk-rs/tree/master/near-sdk/compilation_tests)

### 2. Library compatibility

- Rand library doesn't work with wasm32, only in debug mode
  <br/>for wasm32
```rust
use near_sdk::env::random_seed;
pub fn random_seed() -> Vec<u8> {}
```

### 3. Best practice

[check best practice list](https://www.near-sdk.io/best-practices)

```rust
// start from SDK v4
log!("Transferred {} tokens from {} to {}", amount, sender_id, receiver_id);
```

### 4. Smart Contracts initialization

- use [init] macros

### 5. Testing Smart Contracts

- see **test.sh** and **testnet.sh** scripts

### 6. Sandbox testing_env

- [NEAR Simulator & cross-contract testing library](https://github.com/near/near-sdk-rs/tree/master/near-sdk-sim)
  <br/>(haven't tried it yet)