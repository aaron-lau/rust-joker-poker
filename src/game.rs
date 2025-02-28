use std::io::{self, Write};
use crate::deck::Deck;
use crate::player::Player;
use crate::hand::HandRank;

pub enum GameState {
    Betting,
    Drawing,
    ShowDown,
    GameOver,
}

pub struct Game {
    deck: Deck,
    players: Vec<Player>,
    pot: u32,
    current_player: usize,
    state: GameState,
    min_bet: u32,
    num_jokers: u32,  // Add this field
}

impl Game {
    pub fn new(player_names: Vec<String>, starting_chips: u32, min_bet: u32, num_jokers: u32) -> Self {
        let players = player_names
            .into_iter()
            .map(|name| Player::new(name, starting_chips))
            .collect();

        Game {
            deck: Deck::new(num_jokers),
            players,
            pot: 0,
            current_player: 0,
            state: GameState::Betting,
            min_bet,
            num_jokers,
        }
    }
    
    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_min_bet(&self) -> u32 {
        self.min_bet
    }
    
    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn start_round(&mut self) {
        println!("\n=== Starting New Round ===");
        self.deck = Deck::new(self.num_jokers);  // Use stored number of jokers
        self.deck.shuffle();
        self.pot = 0;
        self.state = GameState::Betting;
        
        // Deal cards to players
        for player in &mut self.players {
            player.hand = Some(self.deck.deal(5));
            player.in_round = true;
        }

        self.betting_round();
    }

    fn betting_round(&mut self) {
        println!("\n=== Betting Round ===");
        let mut current_bet = self.min_bet;
        let mut players_acted = 0;

        while players_acted < self.active_players_count() {
            let current_player = self.current_player;
            let player_chips = self.players[current_player].chips;
            let player_name = self.players[current_player].name.clone();
            
            if self.players[current_player].in_round {
                self.show_player_status(current_player);
                
                println!("Current bet: {}", current_bet);
                println!("1: Call/Check");
                println!("2: Raise");
                println!("3: Fold");
                
                let choice = self.get_player_choice(1..=3);
                match choice {
                    1 => {
                        if let Ok(bet) = self.place_bet(current_player, current_bet) {
                            self.pot += bet;
                            println!("{} calls {}", player_name, bet);
                        } else {
                            self.players[current_player].in_round = false;
                            println!("{} folds (insufficient chips)", player_name);
                        }
                    },
                    2 => {
                        let raise = self.get_player_choice(current_bet..=player_chips);
                        if let Ok(bet) = self.place_bet(current_player, raise) {
                            self.pot += bet;
                            current_bet = raise;
                            println!("{} raises to {}", player_name, raise);
                        }
                    },
                    3 => {
                        self.players[current_player].in_round = false;
                        println!("{} folds", player_name);
                    },
                    _ => unreachable!(),
                }
                players_acted += 1;
            }
            
            self.next_player();
        }

        self.state = GameState::Drawing;
        self.drawing_round();
    }
    

    fn drawing_round(&mut self) {
        println!("\n=== Drawing Round ===");
        for player_idx in 0..self.players.len() {
            if !self.players[player_idx].in_round {
                continue;
            }

            self.show_player_status(player_idx);
            let player_name = self.players[player_idx].name.clone();
            
            println!("How many cards to discard? (0-3):");
            let num_discard = self.get_player_choice(0..=3);
            
            if num_discard > 0 {
                println!("{} discards {} cards", player_name, num_discard);
            }
        }
        
        self.state = GameState::ShowDown;
        self.showdown();
    }

    fn showdown(&mut self) {
        println!("\n=== Showdown ===");
        let mut best_rank = HandRank::HighCard;
        let mut winners = Vec::new();

        // Find the winning hand(s)
        for (i, player) in self.players.iter().enumerate() {
            if player.in_round {
                if let Some(hand) = &player.hand {
                    println!("{}'s hand:", player.name);
                    // Display hand here
                    
                    let rank = hand.evaluate();
                    match rank {
                        r if r > best_rank => {
                            best_rank = r;
                            winners.clear();
                            winners.push(i);
                        }
                        r if r == best_rank => {
                            winners.push(i);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Distribute pot to winners
        let win_amount = self.pot / winners.len() as u32;
        for &winner_idx in &winners {
            let winner = &mut self.players[winner_idx];
            winner.chips += win_amount;
            println!("{} wins {} chips!", winner.name, win_amount);
        }

        self.state = GameState::GameOver;
    }

    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    fn active_players_count(&self) -> usize {
        self.players.iter().filter(|p| p.in_round).count()
    }

    fn show_player_status(&self, player_idx: usize) {
        let player = &self.players[player_idx];
        println!("\n{}'s turn", player.name);
        println!("Chips: {}", player.chips);
        if let Some(hand) = &player.hand {
            println!("Hand: {:?}", hand.cards);
        }
    }

    fn place_bet(&mut self, player_idx: usize, amount: u32) -> Result<u32, &'static str> {
        self.players[player_idx].place_bet(amount)
    }

    fn get_player_choice(&self, range: std::ops::RangeInclusive<u32>) -> u32 {
        loop {
            print!("Enter your choice ({}..{}): ", range.start(), range.end());
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            if let Ok(choice) = input.trim().parse::<u32>() {
                if range.contains(&choice) {
                    return choice;
                }
            }
            println!("Invalid choice, try again");
        }
    }

    pub fn is_game_over(&self) -> bool {
        matches!(self.state, GameState::GameOver)
    }
}