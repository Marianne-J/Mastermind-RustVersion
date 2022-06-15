pub mod master;

use master::Master;

fn main() {
    //Create instance of Master
    master = Master{last_guess:String::from(""), number_of_players:2, game_running:true,
        board:Board{code:String::from(""), guess:String::from(""), player_1_status:[String::from(""),String::from("")],player_2_status:[String::from(""),String::from("")], turn:0},
        console:Console{},
        roster:Roster{players:[Player{player_move:Move{guess:String::from("")}, name:String::from("")}, Player{player_move:Move{guess:String::from("")}, name:String::from("")}], current:0, add_player_index:0}
    }
    
    //Start the game
    start_game();
}