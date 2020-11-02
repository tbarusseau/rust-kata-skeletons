use std::io::{self, BufRead};

pub enum Action<'a> {
    InsertMoney(f32),
    ReturnMoney,
    GetItem(&'a str),
    Service,
}

impl<'a> Action<'a> {
    /// Transform the user input into an action
    /// Option documentation: https://doc.rust-lang.org/stable/std/option/index.html
    pub fn from(s: &'a str) -> Option<Action> {
        unimplemented!()
    }
}

/// Transform a whole input line into actions
/// Vec documentation: https://doc.rust-lang.org/stable/std/vec/index.html
pub fn parse_actions<'a>(input: &'a str) -> Vec<Action> {
    unimplemented!()
}

struct Item<'a> {
    name: &'a str,
    value: f32,
    identifier: &'a str,
}

impl<'a> Item<'a> {
    fn from(name: &'a str, value: f32, identifier: &'a str) -> Self {
        Item {
            name,
            value,
            identifier,
        }
    }
}

pub struct VendingMachine<'a> {
    items: Vec<Item<'a>>,
    inserted_money: f32,
}

impl VendingMachine<'_> {
    pub fn display_instructions(&self) {
        // Clear the terminal screen
        println!("\x1B[2J\x1B[1;1H");

        // TODO
    }

    pub fn new() -> Self {
        VendingMachine {
            items: vec![
                Item::from("Canned soup", 1.50, "A"),
                Item::from("Banana", 0.30, "B"),
                Item::from("Ramen noodles", 0.75, "C"),
                Item::from("Carrot", 0.25, "D"),
            ],
            inserted_money: 0.0,
        }
    }

    pub fn _insert_money() {
        // TODO:

        unimplemented!()
    }

    pub fn _return_money() {
        // TODO

        unimplemented!()
    }

    pub fn _select_item() {
        unimplemented!()
    }

    pub fn _service() {
        unimplemented!()
    }
}

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
