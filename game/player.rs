
/*An object that stores a player's information.*/
struct Player {
   mut player_move:Move, name:String
}

impl Player {
    fn get_move (&self) -> &self.player_move {
        /*Returns the player's current move.*/
        return self.player_move;
    }

    fn get_name (&self) -> &self.player_move {
        /*Returns the player's name.*/
        return self.name;
    }

    fn set_move (&self, &new_move: Move) {
        /*Sets the player's current move.*/
        self.player_move = new_move;
    }

    fn set_name (&self, &new_name: String) {
        /*Sets the player's name.*/
        self.name = new_name;
    }
}