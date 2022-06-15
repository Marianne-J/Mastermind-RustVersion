pub mod console;
pub mod board;
pub mod move;
pub mod player;
pub mod roster;

/*Starts the game and controls whether or not the game ends. Also
passes information between structures.*/
struct Master {
    mut last_guess:String, mut number_of_players:isize, mut game_running:bool, board:Board, console:Console, roster:Roster
}

impl Master {
    fn start_game (&self) {
        /*Starts the game.*/
        prepare_game();
        
        while game_running == true {
            do_outputs();
            get_inputs();
            do_updates();
        }
    }

    fn prepare_game (&self) {
        /*Prepares the game.*/
        for i in 0..number_of_players {
            //Set player names and moves
            let mut name = console.read(String::from("Enter a name for player " + i.to_string() + ": "));
            let mut new_player = Player{player_move:Move{guess:String::from("")}, name:String::from("")};
            new_player.set_name(name);
            move_object = Move{guess:String::from("")};
            move_object.set_guess("----");
            new_player.set_move(move_object);

            //Add player to the roster
            roster.add_player(new_player);
        }
    }

    fn do_outputs (&self) {
        /*Calls the console to display the current board.*/
        console.write(String::from("\n" + &board.board_to_string(roster.players)));
    }

    fn get_inputs (&self) {
        /*Calls the console to take the current player's input.*/
        let mut current_player = roster.get_current();

        console.write(String::from(&current_player.get_name() + "'s turn:"));
        last_guess = console.read("What is your guess? ");
    }

    fn do_updates (&self) {
        /*Updates the game based on the player's input.*/
        //Get the current player
        let mut current_player = roster.get_current();

        //Set the player's move and call Board to apply changes
        let mut move_object = Move{guess:String::from("")};
        move_object.set_guess(&last_guess);
        current_player.set_move(&move_object);
        board.apply(&move_object, &roster.current);

        //Check if the game is over
        if board.check_win() == true {
            game_running = false;
            console.write("\n" + &current_player.get_name() + " wins!");
        }

        //Move to the next player
        roster.next_player();
    }
}