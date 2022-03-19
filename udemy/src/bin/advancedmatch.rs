enum Discount {
    Percent(i32),
    Flat(i32)
}

struct Ticket {
    event: String,
    price: i32
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(mounter) => println!("Discount of {:?}", mounter),
        _ =>(),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50.0,
    };
    match concert {
        Ticket {price: 50, event} => println!("event @ 50 = {:?}", event),
        Ticket {price, ..} => println!("price = {:?}", price),
        _ => (),
    }
}