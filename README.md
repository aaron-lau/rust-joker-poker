# Joker Poker

A command-line implementation of Joker Poker written in Rust. This game features configurable joker counts, multiple players, and comprehensive hand evaluation.

## Description

Joker Poker is a variant of poker that uses a standard 52-card deck plus jokers. The jokers are wild cards that can substitute for any card to make the best possible hand. This implementation includes:

- 🃏 Configurable number of jokers (0-4)
- 👥 Support for multiple players (2-6)
- 💰 Betting system with chips
- 🎲 Complete poker hand evaluation
- 🔄 Multiple round support
- ⚡ Error handling and input validation
- 📊 Player statistics tracking

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (1.56.0 or later)
- Cargo (comes with Rust)

You can install Rust and Cargo by following the instructions at [rustup.rs](https://rustup.rs/)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/aaron-lau/rusty-joker-poker.git
cd joker-poker
```

2. Build the project:

```bash
cargo build --release
```

## Running the Application

To run the game:

```bash
cargo run
```

Or, after building, you can run the executable directly:

```bash
./target/release/joker-poker
```

## Game Rules
1. Each player is dealt 5 cards
2. Players can bet, call, raise, or fold
3. Players can discard up to 3 cards (4 if holding an ace)
4. Jokers can substitute for any card
5. Standard poker hand rankings apply, with the addition of joker combinations

### Hand Rankings (from highest to lowest):

- Five of a Kind (only possible with jokers)
- Royal Flush
- Straight Flush
- Four of a Kind
- Full House
- Flush
- Straight
- Three of a Kind
- Two Pair
- One Pair
- High Card

### Gameplay Flow

- Initial betting round
- Card drawing phase (up to 3 cards)
- Final betting round
- Showdown


## Project Structure

```
joker_poker/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── card.rs          # Card definitions
│   ├── deck.rs          # Deck management
│   ├── hand.rs          # Hand evaluation
│   ├── player.rs        # Player logic
│   ├── game.rs          # Game mechanics
│   └── error.rs         # Error handling
├── tests/               # Integration tests
│   ├── card_tests.rs
│   ├── hand_tests.rs
│   └── game_tests.rs
├── Cargo.lock
└── README.md
```

## Running Tests

To run the test suite:

```
cargo test
```

To run specific test file:

```
cargo test --test hand_tests
```

To run tests with output:

```
cargo test -- --nocapture
```

Development

To run the project in development mode with debug information:

```
cargo run --debug
```

To check for any compilation issues:
```
cargo check
```
To format the code:
```
cargo fmt
```
To check for any linting issues:
```
cargo clippy
```

## Future Improvements

- [ ] Add more unit tests
- [ ] Add graphical user interface
- [ ] Implement network multiplayer
- [ ] Add AI opponents
- [ ] Add tournament mode
- [ ] Implement save/load functionality
- [ ] Add player statistics tracking
- [ ] Implement different poker variants
