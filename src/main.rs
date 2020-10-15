use std::io::{self, BufRead};

fn print_prompt() {
    // Clear the terminal screen
    print!("\x1B[2J\x1B[1;1H");

    println!("Welcome to my rusty vending machine!
Valid set of actions:
- Insert money:
    + NICKEL (0.05)
    + DIME (0.10)
    + QUARTER (0.25)
    + DOLLAR (1.00)
- Return all inserted money:
    + COIN RETURN
- Buy an item:
    + A ($0.65)
    + B ($1)
    + C ($1.50)
- A service person opens the machine and sets the available change and items:
    + SERVICE

Enter your actions, separated by commas:");
}

fn main() {
    print_prompt();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print_prompt();

        let replaced_line = line.unwrap().trim().replace(", ", ",");
        let tokens: Vec<&str> = replaced_line.split(",").collect();

        println!("Here are your tokens, use them as you please: {:?}", tokens);
    }
}
