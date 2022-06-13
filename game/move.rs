
/*An object that stores the player's current guess.*/
struct Move {
    mut guess:String
}

impl Move {
    fn get_guess (&self) -> &self.guess{
        /*Returns the player's current guess.*/
        return self.guess;
    }

    fn set_guess (&self, &input: String) {
        /*Sets the player's current guess.*/
        self.guess = input;
    }
}