// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::io;

struct Bill {
    name: String,
    amount: f64
}

struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {inner: vec![]}
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(Bill);
    }

    fn get_all(&self) -> Vec<Bill> {
        self.inner
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Plaese enter your data again!");
    }
    buffer.trim().to_owned();
}

fn get_bill_amount() -> f64 {
    println!("Amount: ");
    loop {
        let input: String = get_input();
        let parsed_input: Result<f64, _> = parse_input();
        match parsed_input {
            Ok(amount) => return amount,
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    let name = get_input();
    let amount = get_bill_amount();
    let bill = Bill {name, amount};
    bills.add(bill);
    println!("Bill added");
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bill");
        println!("");
        println!("Enter selection: ");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            _ => break,
        }
    }
}

fn main() {
    loop {

    }
}
