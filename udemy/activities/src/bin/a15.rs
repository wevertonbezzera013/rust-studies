// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}


fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage("Billy".to_owned(), 50.0),
        Ticket::Standard(15.0),
        Ticket::Vip("Amy".to_owned(), 30.0),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => println!("Holder: {:?}, price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Price: {:?}", price),
            Ticket::Vip(price, holder) => println!("Holder: {:?}, price: {:?}", holder, price),
            _ =>(),
        }
    }
}
