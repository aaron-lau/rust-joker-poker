use crate::hand::Hand;

pub struct Player {
    pub name: String,
    pub chips: u32,
    pub hand: Option<Hand>,
    pub in_round: bool,
    pub wins: u32,
    pub hands_played: u32,
}

impl Player {
    pub fn new(name: String, starting_chips: u32) -> Self {
        Player {
            name,
            chips: starting_chips,
            hand: None,
            in_round: true,
            wins: 0,
            hands_played: 0,
        }
    }

    pub fn place_bet(&mut self, amount: u32) -> Result<u32, &'static str> {
        if amount > self.chips {
            return Err("Insufficient chips");
        }
        self.chips -= amount;
        Ok(amount)
    }


    pub fn add_chips(&mut self, amount: u32) {
        self.chips += amount;
    }

    pub fn remove_chips(&mut self, amount: u32) -> Result<(), &'static str> {
        if amount > self.chips {
            return Err("Insufficient chips");
        }
        self.chips -= amount;
        Ok(())
    }

    // Optional statistics tracking methods
    pub fn add_win(&mut self) {
        self.wins += 1;
    }

    pub fn add_hand_played(&mut self) {
        self.hands_played += 1;
    }

    pub fn get_wins(&self) -> u32 {
        self.wins
    }

    pub fn get_hands_played(&self) -> u32 {
        self.hands_played
    }

    pub fn get_win_rate(&self) -> f64 {
        if self.hands_played == 0 {
            return 0.0;
        }
        self.wins as f64 / self.hands_played as f64
    }
}