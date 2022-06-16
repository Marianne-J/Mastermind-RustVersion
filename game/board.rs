extern crate rand;

use rand::Rng;
use super::r#move::Move;
use super::player::Player;

/*Builds the text to be displayed to the terminal.*/
pub struct Board {
    code:String, guess:String, turn:usize, player_1_status:[String; 2], player_2_status:[String; 2]
}

impl Board {
    pub fn apply (mut self, move_input: &Move, turn_input: &usize) {
        /*Applies the move made to the player who made the move.*/
        let set_guess = move_input.get_guess();
        let set_turn = turn_input;
        
        self.guess = set_guess.to_string();
        self.turn = *set_turn;

        if self.turn == 0 {
            self.player_1_status[0] = self.guess;

            let mut hint = String::new();
            for i in 0..4 {
                if self.guess.chars().nth(i).unwrap() == self.code.chars().nth(i).unwrap() {
                    hint.push_str("x");
                } else if self.code.contains(self.guess.chars().nth(i).unwrap()) {
                    hint.push_str("o");
                } else {
                    hint.push_str("*");
                }
            }
            self.player_1_status[1] = hint;
        } else if self.turn == 1 {
            self.player_2_status[0] = self.guess;

            let mut hint = String::new();
            for i in 0..4 {
                if self.guess.chars().nth(i).unwrap() == self.code.chars().nth(i).unwrap() {
                    hint.push_str("x");
                } else if self.code.contains(self.guess.chars().nth(i).unwrap()) {
                    hint.push_str("o");
                } else {
                    hint.push_str("*");
                }
            }
            self.player_2_status[1] = hint;
            }    
        }

    pub fn check_win (&self) -> bool {
        if self.guess == self.code {
            return true;
        } else {
            return false;
        }
    }

    pub fn board_to_string (&self, players: &Vec<Player>) -> String {
        let player_1_concat = "Player ".to_owned() + &players[0].get_name() + ": " + &self.player_1_status[0] + "," + &self.player_1_status[1];
        let player_2_concat = "Player ".to_owned() + &players[1].get_name() + ": " + &self.player_2_status[0] + "," + &self.player_2_status[1];
        
        let player_1 = String::from(player_1_concat);
        let player_2 = String::from(player_2_concat);

        let status = String::from(player_1 + &player_2);
        let current_board = String::from("\n--------------------\n".to_owned() + &status + "--------------------\n");

        return current_board;
    }

    pub fn prepare (mut self) {
        let mut rng = rand::thread_rng();
        
        let generated_code: isize = rng.gen_range(1000..9999);

        self.code = generated_code.to_string();
        self.player_1_status[0] = String::from("----");
        self.player_1_status[1] = String::from("****");
        self.player_2_status[0] = String::from("----");
        self.player_2_status[1] = String::from("****");
    }
}