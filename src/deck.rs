use rand::seq::SliceRandom;
use crate::card::{Card, Suit, Rank};
use crate::hand::Hand;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_jokers: u32) -> Self {
        let mut cards = Vec::new();
        
        // Add standard cards
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
                Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
            ].iter() {
                cards.push(Card::new(*rank, suit, false));
            }
        }
        
        // Add specified number of jokers
        for _ in 0..num_jokers {
            cards.push(Card::new(Rank::Ace, Suit::Hearts, true));
        }
        
        Deck { cards }
    }
    
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    pub fn deal(&mut self, num_cards: usize) -> Hand {
        let cards: Vec<Card> = self.cards.drain(0..num_cards).collect();
        Hand::new(cards)
    }
}