use joker_poker::{Player, Hand, Card, Rank, Suit};

fn create_test_player(name: &str, chips: u32) -> Player {
    Player::new(name.to_string(), chips)
}

fn create_test_hand() -> Hand {
    let cards = vec![
        Card::new(Rank::Ace, Suit::Hearts, false),
        Card::new(Rank::King, Suit::Hearts, false),
        Card::new(Rank::Queen, Suit::Hearts, false),
        Card::new(Rank::Jack, Suit::Hearts, false),
        Card::new(Rank::Ten, Suit::Hearts, false),
    ];
    Hand::new(cards)
}

mod player_creation_tests {
    use super::*;

    #[test]
    fn test_new_player_creation() {
        let player = create_test_player("TestPlayer", 1000);
        assert_eq!(player.name, "TestPlayer");
        assert_eq!(player.chips, 1000);
        assert!(player.hand.is_none());
        assert!(player.in_round);
    }

    #[test]
    fn test_player_with_zero_chips() {
        let player = create_test_player("BrokePlayer", 0);
        assert_eq!(player.chips, 0);
    }

    #[test]
    fn test_multiple_players_unique() {
        let player1 = create_test_player("Player1", 1000);
        let player2 = create_test_player("Player2", 1000);
        assert_ne!(player1.name, player2.name);
    }
}

mod chip_management_tests {
    use super::*;

    #[test]
    fn test_successful_bet() {
        let mut player = create_test_player("BettingPlayer", 1000);
        let bet_result = player.place_bet(500);
        assert!(bet_result.is_ok());
        assert_eq!(bet_result.unwrap(), 500);
        assert_eq!(player.chips, 500);
    }

    #[test]
    fn test_insufficient_chips_bet() {
        let mut player = create_test_player("PoorPlayer", 100);
        let bet_result = player.place_bet(500);
        assert!(bet_result.is_err());
        assert_eq!(player.chips, 100); // Chips shouldn't change on failed bet
    }

    #[test]
    fn test_exact_chips_bet() {
        let mut player = create_test_player("AllInPlayer", 1000);
        let bet_result = player.place_bet(1000);
        assert!(bet_result.is_ok());
        assert_eq!(player.chips, 0);
    }

    #[test]
    fn test_zero_bet() {
        let mut player = create_test_player("ZeroBetPlayer", 1000);
        let bet_result = player.place_bet(0);
        assert!(bet_result.is_ok());
        assert_eq!(player.chips, 1000);
    }

    #[test]
    fn test_multiple_bets() {
        let mut player = create_test_player("MultiBetPlayer", 1000);
        assert!(player.place_bet(300).is_ok());
        assert!(player.place_bet(300).is_ok());
        assert!(player.place_bet(300).is_ok());
        assert!(player.place_bet(300).is_err());
        assert_eq!(player.chips, 100);
    }
}

mod hand_management_tests {
    use super::*;

    #[test]
    fn test_assign_hand() {
        let mut player = create_test_player("HandPlayer", 1000);
        let hand = create_test_hand();
        player.hand = Some(hand);
        assert!(player.hand.is_some());
    }

    #[test]
    fn test_clear_hand() {
        let mut player = create_test_player("ClearHandPlayer", 1000);
        player.hand = Some(create_test_hand());
        player.hand = None;
        assert!(player.hand.is_none());
    }

    #[test]
    fn test_replace_hand() {
        let mut player = create_test_player("ReplaceHandPlayer", 1000);
        let initial_hand = create_test_hand();
        player.hand = Some(initial_hand);
        
        let new_hand = Hand::new(vec![
            Card::new(Rank::Two, Suit::Hearts, false),
            Card::new(Rank::Three, Suit::Hearts, false),
            Card::new(Rank::Four, Suit::Hearts, false),
            Card::new(Rank::Five, Suit::Hearts, false),
            Card::new(Rank::Six, Suit::Hearts, false),
        ]);
        player.hand = Some(new_hand);
        
        assert!(player.hand.is_some());
        // Add more specific hand comparison tests if Hand implements PartialEq
    }
}

mod round_participation_tests {
    use super::*;

    #[test]
    fn test_initial_round_status() {
        let player = create_test_player("NewPlayer", 1000);
        assert!(player.in_round);
    }

    #[test]
    fn test_fold_round() {
        let mut player = create_test_player("FoldingPlayer", 1000);
        player.in_round = false;
        assert!(!player.in_round);
    }

    #[test]
    fn test_rejoin_round() {
        let mut player = create_test_player("RejoinPlayer", 1000);
        player.in_round = false;
        player.in_round = true;
        assert!(player.in_round);
    }
}

mod player_state_tests {
    use super::*;

    #[test]
    fn test_player_is_active() {
        let player = create_test_player("ActivePlayer", 1000);
        assert!(player.chips > 0 && player.in_round);
    }

    #[test]
    fn test_player_is_broke() {
        let player = create_test_player("BrokePlayer", 0);
        assert_eq!(player.chips, 0);
    }

    #[test]
    fn test_player_state_after_all_in() {
        let mut player = create_test_player("AllInPlayer", 1000);
        let _ = player.place_bet(1000); // All-in bet
        assert_eq!(player.chips, 0);
        assert!(player.in_round); // Should still be in round after all-in
    }
}

mod error_handling_tests {
    use super::*;

    #[test]
    fn test_negative_chips_prevention() {
        let mut player = create_test_player("NegativeChipsPlayer", 100);
        let bet_result = player.place_bet(101);
        assert!(bet_result.is_err());
        assert_eq!(player.chips, 100);
    }

    #[test]
    fn test_multiple_failed_bets() {
        let mut player = create_test_player("FailedBetsPlayer", 100);
        assert!(player.place_bet(50).is_ok());
        assert!(player.place_bet(60).is_err());
        assert_eq!(player.chips, 50);
    }
}

mod chip_modification_tests {
    use super::*;

    #[test]
    fn test_add_chips() {
        let mut player = create_test_player("ChipAddPlayer", 1000);
        player.add_chips(500);
        assert_eq!(player.chips, 1500);
    }

    #[test]
    fn test_remove_chips() {
        let mut player = create_test_player("ChipRemovePlayer", 1000);
        assert!(player.remove_chips(500).is_ok());
        assert_eq!(player.chips, 500);
    }

    #[test]
    fn test_chip_operations_sequence() {
        let mut player = create_test_player("ChipSeqPlayer", 1000);
        player.add_chips(500);
        assert!(player.remove_chips(300).is_ok());
        player.add_chips(200);
        assert_eq!(player.chips, 1400);
    }
}

mod player_statistics_tests {
    use super::*;

    #[test]
    fn test_track_wins() {
        let mut player = create_test_player("WinningPlayer", 1000);
        player.add_win();
        assert_eq!(player.get_wins(), 1);
    }

    #[test]
    fn test_track_hands_played() {
        let mut player = create_test_player("HandTrackPlayer", 1000);
        player.add_hand_played();
        assert_eq!(player.get_hands_played(), 1);
    }

    #[test]
    fn test_calculate_win_rate() {
        let mut player = create_test_player("WinRatePlayer", 1000);
        player.add_win();
        player.add_hand_played();
        player.add_hand_played();
        assert_eq!(player.get_win_rate(), 0.5);
    }
}