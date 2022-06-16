use super::r#move::Move;

/*An object that stores a player's information.*/
pub struct Player {
   player_move:Move, name:String
}

impl Player {
    pub fn get_move (&self) -> &Move {
        /*Returns the player's current move.*/
        return &self.player_move;
    }

    pub fn get_name (&self) -> &String {
        /*Returns the player's name.*/
        return &self.name;
    }

    pub fn set_move (mut self, new_move: &Move) {
        /*Sets the player's current move.*/
        let set_new_move = new_move;
        self.player_move = *set_new_move;
    }

    pub fn set_name (mut self, new_name: &String) {
        /*Sets the player's name.*/
        let set_new_name = new_name;
        self.name = set_new_name.to_string();
    }
}