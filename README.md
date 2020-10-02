# Game Logic Pallet

## Purpose

This is an example pallet for game logic on the blockchain.
The idea behind this pallet is that you can start the game and its
scores are tracked on the chain.

API looks like this:

```Rust
// Only Root user can start the game
fn start_new_game(origin) -> DispatchResult {}

// Add score - user's address
fn add_score(origin, new_score: u64) -> DispatchResult {}

// Only Root user can end the game
fn end_current_game(origin) -> DispatchResult {}
```

Upon end of the game, the winner is chosen and stored on the chain,
The current game is ended and points are reset, only points that are persisted
on the chain are the records of the winners.

### TODO
- [x] Write game logic
- [x] Write tests
- [ ] Implement RPC call to query the call for all teh winners and their points
- [ ] Play with offchain workers (just because)

## Dependencies

### Traits

This pallet does not depend on any externally defined traits.

### Pallets

This pallet does not depend on any other FRAME pallet or externally developed modules.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-game-logic]
default_features = false
git = 'https://github.com/substrate-developer-hub/substrate-pallet-template.git'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-game-logic/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for test_module
impl pallet_game_logic::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
ExamplePallet: substrate_pallet_template::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
