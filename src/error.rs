use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid bet: {0}")]
    InvalidBet(String),

    #[error("Insufficient chips: needed {needed}, had {available}")]
    InsufficientChips {
        needed: u32,
        available: u32,
    },

    #[error("Invalid game state: {current:?}, expected {expected:?}")]
    InvalidGameState {
        current: GameState,
        expected: GameState,
    },

    #[error("Invalid player action: {0}")]
    InvalidAction(String),

    #[error("Player not found: {0}")]
    PlayerNotFound(String),

    #[error("Invalid number of cards: {0}")]
    InvalidCardCount(usize),

    #[error("Card not found in hand: {0}")]
    CardNotFound(String),

    #[error("Invalid draw: {0}")]
    InvalidDraw(String),

    #[error("Game is full: maximum {max} players, attempted to add player {attempted}")]
    GameFull {
        max: usize,
        attempted: String,
    },

    #[error("Invalid number of jokers: {0}")]
    InvalidJokerCount(u32),

    #[error("Deck is empty")]
    EmptyDeck,

    #[error("Round already in progress")]
    RoundInProgress,

    #[error("Round not in progress")]
    NoRoundInProgress,

    #[error("Player not in round: {0}")]
    PlayerNotInRound(String),

    #[error("Invalid bet amount: minimum {min}, maximum {max}, attempted {attempted}")]
    BetOutOfRange {
        min: u32,
        max: u32,
        attempted: u32,
    },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// Implement custom result type
pub type GameResult<T> = Result<T, GameError>;

// Implementation of custom methods for GameError
impl GameError {
    pub fn is_fatal(&self) -> bool {
        matches!(self, GameError::IoError(_) | GameError::EmptyDeck)
    }

    pub fn can_retry(&self) -> bool {
        matches!(
            self,
            GameError::InvalidBet(_) |
            GameError::InsufficientChips { .. } |
            GameError::InvalidAction(_)
        )
    }
}