
/*Takes input from the user through the terminal and displays text to the
terminal*/
struct Console {}

impl Console {
    fn read (&self, &prompt: String) -> String {
        /*Reads user input from the terminal and returns it.*/
        println!(prompt);
        let response = std::io::stdin().read_line(&mut line).unwrap();

        return response
    }

    fn read_num (&self, &prompt: String) -> isize {
        /*Reads user input from the terminal, specifically the user's guess*/
        let guess = std::io::stdin().read_line(&mut line).unwrap();
        
        return guess.parse()::<isize>().unwrap();
    }

    fn write (&self, &prompt: String) {
        /*Displays text to the terminal*/
        std::io::stdout().write(prompt.as_bytes()).unwrap();
    }
}