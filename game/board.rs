
/*Builds the text to be displayed to the terminal.*/
struct Board {
    mut code:String, mut guess:String, mut player_1_status[String;2], mut player_2_status[String;2], mut turn:isize
}

impl Board {
    fn apply (&self, &move_input: Move, &turn_input: isize) {
        /*Applies the move made to the player who made the move.*/
        guess = move_input.get_guess();
        turn = turn_input;

        if turn == 0 {
            player_1_status[0] = guess;

            let mut hint = String::new();
            for i in 0..4 {
                if guess[i] == code[i] {
                    hint.push_str("x");
                } else if code.contains(guess[i]) {
                    hint.push_str("o");
                } else {
                    hint.push_str("*");
                }
            }
            player_1_status[1] = hint;
        } else if turn == 1 {
            player_2_status[0] = guess;

            let mut hint = String::new();
            for i in 0..4 {
                if guess[i] == code[i] {
                    hint.push_str("x");
                } else if code.contains(guess[i]) {
                    hint.push_str("o");
                } else {
                    hint.push_str("*");
                }
            }
            player_2_status[1] = hint;
            }    
        }

    fn check_win (&self) -> bool {
        if guess == code {
            return true;
        } else {
            return false;
        }
    }

    fn board_to_string (&self, &players[Player;2]) -> String {
        let mut player_1 = String::from("Player " + &players[0].get_name() + ": " + &player_1_status[0] + "," + &player_1_status[1] + "\n");
        let mut player_2 = String::from("Player " + &players[1].get_name() + ": " + &player_2_status[0] + "," + &player_2_status[1] + "\n");

        let mut status = String::from(player_1 + player_2);
        let mut current_board = String::from("\n--------------------\n" + status + "--------------------\n");

        return current_board;
    }

    fn prepare (&self) {
        let mut rng = rand::thread_rng();
        let mut generated_code = rng.gen_range(1000..9999);

        code = generated_code;
        player_1_status[0] = String::from("----");
        player_1_status[1] = String::from("****");
        player_2_status[0] = String::from("----");
        player_2_status[1] = String::from("****");
    }
}