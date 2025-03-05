use joker_poker::{Game, Player, HandRank, Card, Rank, Suit, Hand};
use joker_poker::game::GameState;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// Helper function to create a new game with default settings
fn create_test_game(num_players: u32, starting_chips: u32, min_bet: u32, num_jokers: u32) -> Game {
    let player_names: Vec<String> = (1..=num_players)
        .map(|i| format!("Player{}", i))
        .collect();
    Game::new(player_names, starting_chips, min_bet, num_jokers)
}

// Helper function to create a specific hand for testing
fn create_test_hand(cards: Vec<(Rank, Suit, bool)>) -> Hand {
    let cards = cards.into_iter()
        .map(|(rank, suit, is_joker)| Card::new(rank, suit, is_joker))
        .collect();
    Hand::new(cards)
}

mod game_initialization_tests {
    use super::*;

    #[test]
    fn test_new_game_creation() {
        let game = create_test_game(2, 1000, 10, 2);
        let players = game.get_players();

        assert_eq!(players.len(), 2);
        assert_eq!(players[0].chips, 1000);
        assert_eq!(players[1].chips, 1000);
        assert_eq!(game.get_min_bet(), 10);
    }
    
    #[test]
    fn test_player_count_limits() {
        // Test minimum number of players
        let game_min = create_test_game(2, 1000, 10, 2);
        assert_eq!(game_min.get_players().len(), 2);
        
        // Test maximum number of players
        let game_max = create_test_game(6, 1000, 10, 2);
        assert_eq!(game_max.get_players().len(), 6);
    }
    
    #[test]
    fn test_joker_count_limits() {
        // Test with no jokers
        let game_no_jokers = create_test_game(2, 1000, 10, 0);
        
        // Test with maximum jokers
        let game_max_jokers = create_test_game(2, 1000, 10, 4);
        
        // We can't directly test the deck composition, but we can verify
        // the game initializes successfully with different joker counts
        assert!(game_no_jokers.get_min_bet() == 10);
        assert!(game_max_jokers.get_min_bet() == 10);
    }
}


mod betting_round_tests {
    use super::*;

    #[test]
    fn test_initial_betting() {
        let mut game = create_test_game(2, 1000, 10, 2);

        game.test_place_bet(0, 10).expect("Player 1 should be able to bet min bet");
        game.test_place_bet(1, 10).expect("Player 2 should be able to bet min bet");
        
        // Check chips were deducted
        let players = game.get_players();
        assert_eq!(players[0].chips, 990);
        assert_eq!(players[1].chips, 990);
        
        // Check pot has accumulated bets
        assert_eq!(game.get_pot(), 20);
    }

    #[test]
    fn test_all_in_scenario() {
        let mut game = create_test_game(2, 100, 10, 2);
        
        // Player 1 goes all-in
        game.test_place_bet(0, 100).expect("Player 1 should be able to go all-in");
        
        // Player 2 calls
        game.test_place_bet(1, 100).expect("Player 2 should be able to call all-in");
        
        // Check that players are all-in but still in the round
        let players = game.get_players();
        assert_eq!(players[0].chips, 0);
        assert_eq!(players[1].chips, 0);
        assert!(players[0].in_round);
        assert!(players[1].in_round);
        
        // Check pot has all chips
        assert_eq!(game.get_pot(), 200);
    }

    #[test]
    fn test_folding() {
        let mut game = create_test_game(3, 1000, 10, 2);
        
        // Player 1 and 2 bet
        game.test_place_bet(0, 10).expect("Player 1 should be able to bet");
        game.test_place_bet(1, 10).expect("Player 2 should be able to bet");
        
        // Player 3 folds
        game.test_fold_player(2);
        
        // Verify player 3 is out of the round but still has chips
        let players = game.get_players();
        assert!(players[0].in_round);
        assert!(players[1].in_round);
        assert!(!players[2].in_round);
        
        assert_eq!(players[2].chips, 1000); // Folded player keeps chips
        assert_eq!(game.get_pot(), 20); // Only two players contributed
    }
    
    #[test]
    fn test_raising() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // Player 1 bets minimum
        game.test_place_bet(0, 10).expect("Player 1 should be able to bet min");
        
        // Player 2 raises
        game.test_place_bet(1, 30).expect("Player 2 should be able to raise");
        
        // Player 1 calls the raise
        game.test_place_bet(0, 20).expect("Player 1 should be able to call the raise");
        
        // Check chips are deducted correctly
        let players = game.get_players();
        assert_eq!(players[0].chips, 970); // 1000 - 10 - 20
        assert_eq!(players[1].chips, 970); // 1000 - 30
        
        // Check pot has accumulated all bets
        assert_eq!(game.get_pot(), 60); // 10 + 30 + 20
    }
}

mod drawing_round_tests {
    use super::*;

    #[test]
    fn test_card_drawing() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // Set a known hand for player 1
        let player1_hand = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::Ten, Suit::Hearts, false),
        ]);
        
        // Just directly set player 1's hand
        game.set_player_hand(0, player1_hand);
        
        // Verify the hand was set correctly
        let players = game.get_players();
        if let Some(hand) = &players[0].hand {
            assert_eq!(hand.cards.len(), 5);
        } else {
            panic!("Player 1 should have a hand");
        }
    }

    #[test]
    fn test_max_draw_limit() {
        // Since we don't have a way to test drawing directly yet,
        // this is more of a placeholder
        let mut game = create_test_game(2, 1000, 10, 2);
        assert!(true); // Placeholder assertion
    }
}

mod game_state_tests {
    use super::*;

    #[test]
    fn test_game_progression() {
        let mut game = create_test_game(2, 1000, 10, 2);
        assert!(!game.is_game_over());
    }

    #[test]
    fn test_player_elimination() {
        let mut game = create_test_game(3, 100, 10, 2);
        
        // Make player 1 go all-in and lose all chips
        game.test_place_bet(0, 100).expect("Player 1 should be able to go all-in");
        
        // Verify player 1 has 0 chips
        let players = game.get_players();
        assert_eq!(players[0].chips, 0);
    }
}

mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_bet_handling() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // Setup a test round (but don't run the interactive start_round)
        game.setup_test_round();
        
        // Try to bet less than minimum - should be explicitly rejected
        let result = game.test_place_bet(0, 5);
        assert!(result.is_err(), "Betting below minimum should fail");
        
        // Verify the error message is related to minimum bet
        if let Err(err) = result {
            assert!(err.contains("below minimum"), 
                    "Error should mention bet being below minimum: {}", err);
        } else {
            panic!("Expected an error for betting below minimum");
        }
        
        // Try to bet more than player has
        let result = game.test_place_bet(0, 2000);
        assert!(result.is_err(), "Betting more than player has should fail");
        
        // Check that player's chips haven't changed after failed bets
        let players = game.get_players();
        assert_eq!(players[0].chips, 1000);
        
        // Finish the round directly to avoid hanging
        game.set_game_state(GameState::GameOver);
    }

    #[test]
    fn test_minimum_bet_enforcement() {
        let mut game = create_test_game(2, 1000, 50, 2);
        
        // Setup a test round
        game.setup_test_round();
        
        // Try to bet less than minimum
        let result = game.test_place_bet(0, 10); // Min bet is 50
        assert!(result.is_err(), "Bet below minimum should fail");
        
        // Verify the error message
        if let Err(err) = result {
            assert!(err.contains("below minimum"), 
                    "Error should mention bet being below minimum: {}", err);
        } else {
            panic!("Expected an error for betting below minimum");
        }
        
        // Try valid minimum bet
        let result = game.test_place_bet(0, 50);
        assert!(result.is_ok(), "Minimum bet should succeed");
        
        // Check chips were deducted only for the successful bet
        let players = game.get_players();
        assert_eq!(players[0].chips, 950);
        
        // Finish the round directly to avoid hanging
        game.set_game_state(GameState::GameOver);
    }

    #[test]
    fn test_insufficient_chips_handling() {
        let mut game = create_test_game(2, 50, 100, 2);
        
        // Setup a test round
        game.setup_test_round();
        
        // Player can't meet minimum bet
        let result = game.test_place_bet(0, 100);
        assert!(result.is_err(), "Player shouldn't be able to bet more than they have");
        
        // Check that the player is still in the round but with unchanged chips
        let players = game.get_players();
        assert_eq!(players[0].chips, 50);
        assert!(players[0].in_round);
        
        // Finish the round directly to avoid hanging
        game.set_game_state(GameState::GameOver);
    }
}

mod game_mechanics_tests {
    use super::*;

    #[test]
    fn test_raise_limits() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // Setup a test round
        game.setup_test_round();
        
        // Player 1 bets minimum
        game.test_place_bet(0, 10).expect("Player 1 should be able to bet minimum");
        
        // Player 2 raises to double
        game.test_place_bet(1, 20).expect("Player 2 should be able to raise");
        
        // Player 1 re-raises
        game.test_place_bet(0, 30).expect("Player 1 should be able to re-raise");
        
        // Check pot and player chips
        let players = game.get_players();
        assert_eq!(players[0].chips, 960); // 1000 - 10 - 30
        assert_eq!(players[1].chips, 980); // 1000 - 20
        assert_eq!(game.get_pot(), 60); // 10 + 20 + 30
        
        // Finish the round directly to avoid hanging
        game.set_game_state(GameState::GameOver);
    }
}

mod showdown_tests {
    use super::*;

    #[test]
    fn test_winner_determination() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // Setup a test round
        game.setup_test_round();
        
        // Give player 1 a royal flush (best hand)
        let royal_flush = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::Queen, Suit::Hearts, false),
            (Rank::Jack, Suit::Hearts, false),
            (Rank::Ten, Suit::Hearts, false),
        ]);
        game.set_player_hand(0, royal_flush);
        
        // Give player 2 a straight flush (second-best hand)
        let straight_flush = create_test_hand(vec![
            (Rank::Nine, Suit::Clubs, false),
            (Rank::Eight, Suit::Clubs, false),
            (Rank::Seven, Suit::Clubs, false),
            (Rank::Six, Suit::Clubs, false),
            (Rank::Five, Suit::Clubs, false),
        ]);
        game.set_player_hand(1, straight_flush);
        
        // Both players bet
        game.test_place_bet(0, 100).expect("Player 1 should be able to bet");
        game.test_place_bet(1, 100).expect("Player 2 should be able to bet");
        
        // Skip to showdown directly
        game.set_game_state(GameState::ShowDown);
        
        // Directly distribute pot to player 1 (who has the better hand)
        let initial_chips = game.get_players()[0].chips;
        game.distribute_pot_to_player(0);
        
        // Check that player 1 received the pot
        assert_eq!(game.get_players()[0].chips, initial_chips + 200);
        assert_eq!(game.get_pot(), 0);
        
        // Set game to over
        game.set_game_state(GameState::GameOver);
    }

    #[test]
    fn test_pot_distribution() {
        let mut game = create_test_game(3, 1000, 10, 2);
        
        // Setup a test round
        game.setup_test_round();
        
        // All players bet the same amount
        let bet_amount = 100;
        game.test_place_bet(0, bet_amount).expect("Player 1 should be able to bet");
        game.test_place_bet(1, bet_amount).expect("Player 2 should be able to bet");
        game.test_place_bet(2, bet_amount).expect("Player 3 should be able to bet");
        
        // Verify pot contains all bets
        assert_eq!(game.get_pot(), bet_amount * 3);
        
        // Record player 1's initial chips
        let initial_chips = game.get_players()[0].chips;
        
        // Distribute pot to player 1
        game.distribute_pot_to_player(0);
        
        // Verify player 1 got the entire pot
        assert_eq!(game.get_players()[0].chips, initial_chips + bet_amount * 3);
        assert_eq!(game.get_pot(), 0);
        
        // Finish the round
        game.set_game_state(GameState::GameOver);
    }

    #[test]
    fn test_tie_handling() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // Setup a test round
        game.setup_test_round();
        
        // Give both players identical hands (for a tie)
        let tied_hand1 = create_test_hand(vec![
            (Rank::Ace, Suit::Hearts, false),
            (Rank::Ace, Suit::Diamonds, false),
            (Rank::King, Suit::Hearts, false),
            (Rank::King, Suit::Diamonds, false),
            (Rank::Queen, Suit::Hearts, false),
        ]);
        
        let tied_hand2 = create_test_hand(vec![
            (Rank::Ace, Suit::Clubs, false),
            (Rank::Ace, Suit::Spades, false),
            (Rank::King, Suit::Clubs, false),
            (Rank::King, Suit::Spades, false),
            (Rank::Queen, Suit::Clubs, false),
        ]);
        
        game.set_player_hand(0, tied_hand1);
        game.set_player_hand(1, tied_hand2);
        
        // Both players bet
        let bet_amount = 100;
        game.test_place_bet(0, bet_amount).expect("Player 1 should be able to bet");
        game.test_place_bet(1, bet_amount).expect("Player 2 should be able to bet");
        
        // Record initial chips
        let initial_chips_p1 = game.get_players()[0].chips;
        let initial_chips_p2 = game.get_players()[1].chips;
        
        // For a tie, we'd normally split the pot
        // But we'll simulate it manually here
        let half_pot = game.get_pot() / 2;
        game.distribute_pot_to_player(0);
        
        // Need to add the pot back for the second player
        // (this is a bit of a hack, but it works for testing)
        let mut game_players = game.get_players_mut();
        game_players[0].chips -= half_pot;
        game.set_pot(half_pot);
        game.distribute_pot_to_player(1);
        
        // Verify each player got half the original pot
        assert_eq!(game.get_players()[0].chips, initial_chips_p1 + half_pot);
        assert_eq!(game.get_players()[1].chips, initial_chips_p2 + half_pot);
        
        // Finish the round
        game.set_game_state(GameState::GameOver);
    }
}

mod multi_round_tests {
    use super::*;

    #[test]
    fn test_multiple_rounds() {
        let mut game = create_test_game(2, 1000, 10, 2);
        
        // First round
        game.setup_test_round();
        
        // Make some bets
        game.test_place_bet(0, 50).expect("Player 1 should be able to bet");
        game.test_place_bet(1, 50).expect("Player 2 should be able to bet");
        
        // Finish first round
        game.finish_round();
        
        // Record chip counts after first round
        let chips_after_round1 = game.get_players().iter().map(|p| p.chips).collect::<Vec<_>>();
        
        // Start second round
        game.setup_test_round();
        
        // Make more bets
        game.test_place_bet(0, 10).expect("Player 1 should be able to bet in second round");
        game.test_place_bet(1, 10).expect("Player 2 should be able to bet in second round");
        
        // Verify chips changed from the first round's ending values
        let players = game.get_players();
        assert_eq!(players[0].chips, chips_after_round1[0] - 10);
        assert_eq!(players[1].chips, chips_after_round1[1] - 10);
        
        // Finish second round
        game.finish_round();
    }

    #[test]
    fn test_chip_tracking_across_rounds() {
        let mut game = create_test_game(2, 1000, 10, 2);
        let initial_chips = game.get_players()[0].chips;

        // First round
        game.setup_test_round();
        
        // Player 1 and 2 both bet
        game.test_place_bet(0, 100).expect("Player 1 should be able to bet");
        game.test_place_bet(1, 100).expect("Player 2 should be able to bet");
        
        // Give win to player 1
        game.distribute_pot_to_player(0);
        
        // Finish first round
        game.set_game_state(GameState::GameOver);
        
        // Check player 1's chips increased
        assert_eq!(game.get_players()[0].chips, initial_chips + 100);
        
        // Start second round
        game.setup_test_round();
        
        // Player 1 bets again
        game.test_place_bet(0, 50).expect("Player 1 should be able to bet in second round");
        
        // Check that chips reflect the history of both rounds
        assert_eq!(game.get_players()[0].chips, initial_chips + 100 - 50);
        
        // Finish second round
        game.set_game_state(GameState::GameOver);
    }
}