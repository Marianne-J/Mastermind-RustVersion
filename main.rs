pub mod game;

use game::master::Master;
use game::board::Board;
use game::console::Console;
use game::roster::Roster;

fn main() {
    //Create instance of Master
    let master = Master{last_guess:String::from(""), number_of_players:2, game_running:true,
        board:Board{code:String::from(""), guess:String::from(""), player_1_status:[String::from(""),String::from("")],player_2_status:[String::from(""),String::from("")], turn:0},
        console:Console{},
        roster:Roster{players:Vec::new(), current:0}
    };
    
    //Start the game
    master.start_game();
}