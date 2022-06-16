
/*Takes input from the user through the terminal and displays text to the
terminal*/
pub struct Console {}

impl Console {
    pub fn read (&self, prompt: &String) -> String {
        /*Reads user input from the terminal and returns it.*/
        println!("{}", prompt);
        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();

        return response
    }

    pub fn write (&self, prompt: &String) {
        /*Displays text to the terminal*/
        println!("{}", prompt);
    }
}