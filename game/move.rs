
/*An object that stores the player's current guess.*/
pub struct Move {
    guess:String
}

impl Move {
    pub fn get_guess (&self) -> &String {
        /*Returns the player's current guess.*/
        return &self.guess;
    }

    pub fn set_guess (mut self, input: &String) {
        /*Sets the player's current guess.*/
        let new_guess = input;
        self.guess = new_guess.to_string();
    }
}