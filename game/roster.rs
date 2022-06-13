
/*Keeps track of the players and the current turn*/
struct Roster {
    mut players[Player;2], mut current:isize, mut add_player_index:isize
}

impl Roster {
    fn add_player (&self, &player: Player) {
        /*Adds player to the roster.*/
        if self.players.len() < 2 {
            self.players[self.add_player_index] = player;
        }
    }

    fn get_current (&self) -> &Player {
        /*Returns the current player.*/
        return self.players[current];
    }

    fn next_player (&self) {
        /*Changes the turn to the next player.*/
        self.current = (self.current + 1) % self.players.len()
    }
}