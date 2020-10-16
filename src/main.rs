mod vending_machine;

use std::io::{self, BufRead};
use vending_machine::VendingMachine;

fn main() {
    let machine = VendingMachine::new();
    machine.display_instructions();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let replaced_line = line.unwrap().trim().replace(", ", ",");
        let tokens: Vec<&str> = replaced_line.split(",").collect();

        machine.display_instructions();
        println!("Here are your tokens, use them as you please: {:?}", tokens);
    }
}
