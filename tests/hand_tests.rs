use joker_poker::{Hand, Card, Rank, Suit, HandRank};

// Helper function to create test hands
fn create_test_hand(cards: Vec<(Rank, Suit, bool)>) -> Hand {
    let cards = cards.into_iter()
        .map(|(rank, suit, is_joker)| Card::new(rank, suit, is_joker))
        .collect();
    Hand::new(cards)
}

mod royal_flush_tests {
    use super::*;

    #[test]
    fn test_natural_royal_flush() {
        let hand = create_test_hand(vec![
            (Rank::Ten, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Ace, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::RoyalFlush);
    }

    #[test]
    fn test_royal_flush_with_one_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ten, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::RoyalFlush);
    }

    #[test]
    fn test_royal_flush_with_two_jokers() {
        let hand = create_test_hand(vec![
            (Rank::Ten, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
            (Rank::King, Suit::Hearts, true), // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::RoyalFlush);
    }
}

mod five_of_a_kind_tests {
    use super::*;

    #[test]
    fn test_five_of_kind_with_one_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, false),
            (Rank::Ace, Suit::Spades, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::FiveOfAKind);
    }

    #[test]
    fn test_five_of_kind_with_two_jokers() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::FiveOfAKind);
    }
}

mod straight_flush_tests {
    use super::*;

    #[test]
    fn test_natural_straight_flush() {
        let hand = create_test_hand(vec![
            (Rank::Five, Suit::Hearts, false),
            (Rank::Six, Suit::Hearts, false),
            (Rank::Seven, Suit::Hearts, false),
            (Rank::Eight, Suit::Hearts, false),
            (Rank::Nine, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::StraightFlush);
    }

    #[test]
    fn test_straight_flush_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Five, Suit::Hearts, false),
            (Rank::Six, Suit::Hearts, false),
            (Rank::Seven, Suit::Hearts, false),
            (Rank::Eight, Suit::Hearts, false),
            (Rank::Nine, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::StraightFlush);
    }

    #[test]
    fn test_ace_low_straight_flush() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Two, Suit::Hearts, false),
            (Rank::Three, Suit::Hearts, false),
            (Rank::Four, Suit::Hearts, false),
            (Rank::Five, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::StraightFlush);
    }
}

mod four_of_a_kind_tests {
    use super::*;

    #[test]
    fn test_natural_four_of_kind() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, false),
            (Rank::Ace, Suit::Spades, false),
            (Rank::King, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::FourOfAKind);
    }

    #[test]
    fn test_four_of_kind_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::FourOfAKind);
    }
}

mod full_house_tests {
    use super::*;

    #[test]
    fn test_natural_full_house() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::King, Suit::Diamonds, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::FullHouse);
    }

    #[test]
    fn test_full_house_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::King, Suit::Diamonds, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::FullHouse);
    }
}

mod flush_tests {
    use super::*;

    #[test]
    fn test_natural_flush() {
        let hand = create_test_hand(vec![
            (Rank::Two, Suit::Hearts, false),
            (Rank::Five, Suit::Hearts, false),
            (Rank::Seven, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::Flush);
    }

    #[test]
    fn test_flush_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Two, Suit::Hearts, false),
            (Rank::Five, Suit::Hearts, false),
            (Rank::Seven, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::Flush);
    }
}

mod straight_tests {
    use super::*;

    #[test]
    fn test_natural_straight() {
        let hand = create_test_hand(vec![
            (Rank::Five, Suit::Hearts, false),
            (Rank::Six, Suit::Diamonds, false),
            (Rank::Seven, Suit::Clubs, false),
            (Rank::Eight, Suit::Hearts, false),
            (Rank::Nine, Suit::Spades, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::Straight);
    }

    #[test]
    fn test_straight_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Five, Suit::Hearts, false),
            (Rank::Six, Suit::Diamonds, false),
            (Rank::Seven, Suit::Clubs, false),
            (Rank::Eight, Suit::Hearts, false),
            (Rank::Nine, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::Straight);
    }

    #[test]
    fn test_ace_low_straight() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Two, Suit::Diamonds, false),
            (Rank::Three, Suit::Clubs, false),
            (Rank::Four, Suit::Hearts, false),
            (Rank::Five, Suit::Spades, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::Straight);
    }

    #[test]
    fn test_straight_with_joker_in_middle() {
        let hand = create_test_hand(vec![
            (Rank::Five, Suit::Hearts, false),
            (Rank::Six, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, true),  // Joker
            (Rank::Eight, Suit::Hearts, false),
            (Rank::Nine, Suit::Hearts, false),  
        ]);
        assert_eq!(hand.evaluate(), HandRank::Straight);
    }
}

mod three_of_a_kind_tests {
    use super::*;

    #[test]
    fn test_natural_three_of_kind() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::Ace, Suit::Clubs, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Diamonds, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_three_of_kind_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Diamonds, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        assert_eq!(hand.evaluate(), HandRank::ThreeOfAKind);
    }
}

mod two_pair_tests {
    use super::*;

    #[test]
    fn test_natural_two_pair() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::King, Suit::Diamonds, false),
            (Rank::Queen, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::TwoPair);
    }

    #[test]
    fn test_two_pair_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::King, Suit::Diamonds, false),
            (Rank::Queen, Suit::Hearts, true),  // Joker
        ]);
        // With a joker and two pairs, this should evaluate to a Full House
        // because that's the best possible hand with these cards
        assert_eq!(hand.evaluate(), HandRank::FullHouse);
    }
}

mod one_pair_tests {
    use super::*;

    #[test]
    fn test_natural_one_pair() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::OnePair);
    }

    #[test]
    fn test_one_pair_with_joker() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::Ace, Suit::Hearts, true),  // Joker
        ]);
        // With a pair and a joker, this should evaluate to Three of a Kind
        // because that's the best possible hand with these cards
        assert_eq!(hand.evaluate(), HandRank::ThreeOfAKind);
    }
}

mod high_card_tests {
    use super::*;

    #[test]
    fn test_high_card() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::King, Suit::Diamonds, false),
            (Rank::Queen, Suit::Clubs, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::Nine, Suit::Spades, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::HighCard);
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn test_all_jokers() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
        ]);
        // Five jokers should make the best possible hand: Five of a Kind
        assert_eq!(hand.evaluate(), HandRank::FiveOfAKind);
    }

    #[test]
    fn test_four_jokers() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
            (Rank::Ace, Suit::Hearts, true),
        ]);
        assert_eq!(hand.evaluate(), HandRank::FiveOfAKind);
    }

    #[test]
    fn test_empty_hand() {
        let hand = create_test_hand(vec![]);
        assert_eq!(hand.evaluate(), HandRank::HighCard);
    }

    #[test]
    fn test_invalid_hand_size() {
        let hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
        ]);
        assert_eq!(hand.evaluate(), HandRank::HighCard);
    }
}