use super::player::Player;

/*Keeps track of the players and the current turn*/
pub struct Roster {
    current:usize, pub players:Vec<Player>
}

impl Roster {
    pub fn add_player (mut self, player: Player) {
        /*Adds player to the roster.*/
        if self.players.len() < 2 {
            self.players.push(player);
        }
    }

    pub fn get_current (&self) -> &Player {
        /*Returns the current player.*/
        return &self.players[self.current];
    }

    pub fn get_current_index (&self) -> &usize {
        /*Returns the index of the current player.*/
        return &self.current
    }

    pub fn next_player (mut self) {
        /*Changes the turn to the next player.*/
        self.current = (self.current + 1) % 2;
    }
}