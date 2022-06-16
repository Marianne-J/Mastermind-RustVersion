use super::board::Board;
use super::console::Console;
use super::roster::Roster;
use super::player::Player;
use super::r#move::Move;

/*Starts the game and controls whether or not the game ends. Also
passes information between structures.*/
pub struct Master {
    last_guess:String, number_of_players:isize, game_running:bool, board:Board, console:Console, roster:Roster
}

impl Master {
    pub fn start_game (&self) {
        /*Starts the game.*/
        self.prepare_game();
        
        while self.game_running == true {
            self.do_outputs();
            self.get_inputs();
            self.do_updates();
        }
    }

    fn prepare_game (&self) {
        /*Prepares the game.*/
        for i in 0..self.number_of_players {
            //Set player names and moves
            let prompt = "Enter a name for player ".to_owned() + &i.to_string() + ": ";
            let name = self.console.read(&String::from(prompt));
            let new_player = Player{player_move:Move{guess:String::from("")}, name:String::from("")};
            new_player.set_name(&name);
            let move_object = Move{guess:String::from("")};
            move_object.set_guess(&String::from("----"));
            new_player.set_move(&move_object);

            //Add player to the roster
            self.roster.add_player(new_player);
        }
    }

    fn do_outputs (&self) {
        /*Calls the console to display the current board.*/
        let prompt = "\n".to_owned() + &self.board.board_to_string(&self.roster.players);
        self.console.write(&String::from(prompt));
    }

    fn get_inputs (mut self) {
        /*Calls the console to take the current player's input.*/
        let current_player = self.roster.get_current();
        let current_player_name = current_player.get_name();
        let prompt = current_player_name.to_owned() + &"'s turn:";

        self.console.write(&String::from(prompt));
        self.last_guess = self.console.read(&String::from("What is your guess? "));
    }

    fn do_updates (mut self) {
        /*Updates the game based on the player's input.*/
        //Get the current player
        let current_player = self.roster.get_current();

        //Set the player's move and call Board to apply changes
        let move_object = Move{guess:String::from("")};
        move_object.set_guess(&self.last_guess);
        current_player.set_move(&move_object);
        self.board.apply(&move_object, self.roster.get_current_index());

        //Check if the game is over
        if self.board.check_win() == true {
            self.game_running = false;
            let prompt = "\n".to_owned() + current_player.get_name() + "wins!";
            self.console.write(&String::from(prompt));
        }

        //Move to the next player
        self.roster.next_player();
    }
}