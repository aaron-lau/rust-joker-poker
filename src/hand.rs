use std::collections::HashMap;
use crate::card::{Card, Rank, Suit};
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,  // Only possible with jokers
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Hand { cards }
    }

    pub fn evaluate(&self) -> HandRank {
        let num_jokers = self.cards.iter().filter(|c| c.is_joker).count();

        // Special case: all jokers
        if num_jokers == 5 {
            return HandRank::FiveOfAKind;
        }

        let non_joker_cards: Vec<&Card> = self.cards.iter().filter(|c| !c.is_joker).collect();

        // Count ranks and suits
        let mut rank_counts: HashMap<Rank, usize> = HashMap::new();
        let mut suit_counts: HashMap<Suit, usize> = HashMap::new();

        for card in &non_joker_cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
            *suit_counts.entry(card.suit).or_insert(0) += 1;
        }

        // Sort rank counts in descending order
        let mut rank_count_vec: Vec<(Rank, usize)> = rank_counts.into_iter().collect();
        rank_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

        // Check for Five of a Kind (requires jokers)
        if !rank_count_vec.is_empty() {
            let highest_count = rank_count_vec[0].1;
            if highest_count + num_jokers >= 5 {
                return HandRank::FiveOfAKind;
            }
        }

        // Check for Royal Flush
        if self.is_royal_flush(num_jokers, &suit_counts, &non_joker_cards) {
            return HandRank::RoyalFlush;
        }

        // Check for Straight Flush
        if self.is_straight_flush(num_jokers, &suit_counts, &non_joker_cards) {
            return HandRank::StraightFlush;
        }

        // Check for Four of a Kind
        if !rank_count_vec.is_empty() && (
            rank_count_vec[0].1 + num_jokers >= 4
        ) {
            return HandRank::FourOfAKind;
        }

        // Check for Full House
        if self.is_full_house(num_jokers, &rank_count_vec) {
            return HandRank::FullHouse;
        }

        // Check for Flush
        if self.is_flush(num_jokers, &suit_counts) {
            return HandRank::Flush;
        }

        // Check for Straight
        if self.is_straight(num_jokers, &non_joker_cards) {
            return HandRank::Straight;
        }

        // Check for Three of a Kind
        if !rank_count_vec.is_empty() && (
            rank_count_vec[0].1 + num_jokers >= 3
        ) {
            return HandRank::ThreeOfAKind;
        }

        // Check for Two Pair
        if self.is_two_pair(num_jokers, &rank_count_vec) {
            return HandRank::TwoPair;
        }

        // Check for One Pair
        if !rank_count_vec.is_empty() && (
            rank_count_vec[0].1 + num_jokers >= 2
        ) {
            return HandRank::OnePair;
        }

        HandRank::HighCard
    }

    fn is_royal_flush(&self, num_jokers: usize, suit_counts: &HashMap<Suit, usize>, non_joker_cards: &[&Card]) -> bool {
        if let Some((&suit, &count)) = suit_counts.iter().max_by_key(|&(_, count)| count) {
            if count + num_jokers >= 5 {
                let royal_ranks = vec![Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
                let mut missing_ranks = royal_ranks.len();

                for card in non_joker_cards {
                    if card.suit == suit && royal_ranks.contains(&card.rank) {
                        missing_ranks -= 1;
                    }
                }

                return missing_ranks <= num_jokers;
            }
        }
        false
    }

    fn is_straight_flush(&self, num_jokers: usize, suit_counts: &HashMap<Suit, usize>, non_joker_cards: &[&Card]) -> bool {
        if let Some((&suit, &count)) = suit_counts.iter().max_by_key(|&(_, count)| count) {
            if count + num_jokers >= 5 {
                let suited_cards: Vec<&Card> = non_joker_cards.iter()
                    .filter(|c| c.suit == suit)
                    .copied()
                    .collect();
                return self.can_form_straight(&suited_cards, num_jokers);
            }
        }
        false
    }

    fn is_full_house(&self, num_jokers: usize, rank_count_vec: &[(Rank, usize)]) -> bool {
        if rank_count_vec.len() >= 2 {
            let highest_count = rank_count_vec[0].1;
            let second_highest_count = rank_count_vec[1].1;

            match num_jokers {
                0 => highest_count >= 3 && second_highest_count >= 2,
                1 => (highest_count >= 3 && second_highest_count >= 1) || 
                     (highest_count >= 2 && second_highest_count >= 2),
                2 => highest_count >= 2 && second_highest_count >= 1,
                _ => true,
            }
        } else {
            false
        }
    }

    fn is_flush(&self, num_jokers: usize, suit_counts: &HashMap<Suit, usize>) -> bool {
        suit_counts.values().any(|&count| count + num_jokers >= 5)
    }

    fn is_straight(&self, num_jokers: usize, non_joker_cards: &[&Card]) -> bool {
        self.can_form_straight(non_joker_cards, num_jokers)
    }

    fn can_form_straight(&self, cards: &[&Card], num_jokers: usize) -> bool {
        if cards.is_empty() && num_jokers < 5 {
            return false;
        }

        let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
        ranks.sort();
        ranks.dedup();

        // Check each possible starting rank
        for &rank in &ranks {
            let rank_val = rank as u8;
            let mut missing_ranks = 0;

            for i in 0..5 {
                let needed_rank = ((rank_val + i) % 13) as u8;
                if !ranks.contains(&Rank::from_u8(needed_rank).unwrap_or(Rank::Two)) {
                    missing_ranks += 1;
                }
            }

            if missing_ranks <= num_jokers {
                return true;
            }
        }

        // Special case: Ace-low straight (A,2,3,4,5)
        if ranks.contains(&Rank::Ace) {
            let low_straight = vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
            let mut missing_ranks = 0;

            for rank in low_straight {
                if !ranks.contains(&rank) {
                    missing_ranks += 1;
                }
            }

            if missing_ranks <= num_jokers {
                return true;
            }
        }

        false
    }

    fn is_two_pair(&self, num_jokers: usize, rank_count_vec: &[(Rank, usize)]) -> bool {
        if rank_count_vec.len() >= 2 {
            match num_jokers {
                0 => rank_count_vec.len() >= 2 && rank_count_vec[0].1 >= 2 && rank_count_vec[1].1 >= 2,
                1 => rank_count_vec[0].1 >= 2 && rank_count_vec[1].1 >= 1,
                _ => true,
            }
        } else {
            false
        }
    }
}

impl Clone for Hand {
    fn clone(&self) -> Self {
        Hand {
            cards: self.cards.clone(),
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hand {{ cards: {:?} }}", self.cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for tests
    fn create_test_cards(ranks: Vec<Rank>, suits: Vec<Suit>, jokers: usize) -> Vec<Card> {
        let mut cards: Vec<Card> = ranks.into_iter()
            .zip(suits.into_iter())
            .map(|(rank, suit)| Card::new(rank, suit, false))
            .collect();
        
        // Add jokers
        for _ in 0..jokers {
            cards.push(Card::new(Rank::Ace, Suit::Hearts, true));
        }
        
        cards
    }

    mod creation_tests {
        use super::*;

        #[test]
        fn test_new_hand() {
            let cards = create_test_cards(
                vec![Rank::Ace],
                vec![Suit::Hearts],
                0
            );
            let hand = Hand::new(cards);
            assert_eq!(hand.cards.len(), 1);
        }
    }

    mod evaluation_tests {
        use super::*;

        #[test]
        fn test_five_of_kind() {
            let cards = create_test_cards(
                vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace],
                vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades],
                1
            );
            let hand = Hand::new(cards);
            assert_eq!(hand.evaluate(), HandRank::FiveOfAKind);
        }

        #[test]
        fn test_royal_flush() {
            let cards = create_test_cards(
                vec![Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace],
                vec![Suit::Hearts, Suit::Hearts, Suit::Hearts, Suit::Hearts, Suit::Hearts],
                0
            );
            let hand = Hand::new(cards);
            assert_eq!(hand.evaluate(), HandRank::RoyalFlush);
        }
    }

    mod joker_tests {
        use super::*;

        #[test]
        fn test_single_joker() {
            let cards = create_test_cards(
                vec![Rank::Ace, Rank::Ace, Rank::Ace],
                vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs],
                1
            );
            let hand = Hand::new(cards);
            assert_eq!(hand.evaluate(), HandRank::FourOfAKind);
        }

        #[test]
        fn test_multiple_jokers() {
            let cards = create_test_cards(
                vec![Rank::Ace, Rank::Ace],
                vec![Suit::Hearts, Suit::Diamonds],
                2
            );
            let hand = Hand::new(cards);
            assert_eq!(hand.evaluate(), HandRank::FourOfAKind);
        }
    }
}