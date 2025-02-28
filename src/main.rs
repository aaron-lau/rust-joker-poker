mod card;
mod deck;
mod hand;
mod player;
mod game;

use std::io::{self, Write};
pub use game::Game;

fn main() {
    println!("Welcome to Joker Poker!");
    
    // Get number of players
    let num_players = get_number_input("Enter number of players (2-6): ", 2..=6);
    
    // Get number of jokers
    let num_jokers = get_number_input("Enter number of jokers (0-4): ", 0..=4);
    
    // Get player names
    let mut player_names = Vec::new();
    for i in 1..=num_players {
        print!("Enter name for Player {}: ", i);
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        player_names.push(name.trim().to_string());
    }
    
    // Initialize game with specified number of jokers
    let mut game = Game::new(
        player_names,
        1000,     // Starting chips
        10,       // Min bet
        num_jokers
    );
    
    // Main game loop
    loop {
        game.start_round();
        
        while !game.is_game_over() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        print!("Play another round? (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().to_lowercase() != "y" {
            break;
        }
        
        // Optionally allow changing number of jokers between rounds
        print!("Change number of jokers? (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().to_lowercase() == "y" {
            let new_num_jokers = get_number_input("Enter new number of jokers (0-4): ", 0..=4);
            game = Game::new(
                game.get_players().iter().map(|p| p.name.clone()).collect(),
                game.get_players()[0].chips,
                game.get_min_bet(),
                new_num_jokers
            );
        }
    }
    
    println!("Thanks for playing!");
}

fn get_number_input(prompt: &str, range: std::ops::RangeInclusive<u32>) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(num) = input.trim().parse::<u32>() {
            if range.contains(&num) {
                return num;
            }
        }
        println!("Invalid input, try again");
    }
}