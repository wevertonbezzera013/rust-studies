// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase



// * Use a struct to store at least the age of a customer
struct Custumer {
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
fn make_restricted(custumer: &Custumer) -> Result<(), String> {
    if custumer.age < 21 {
        Err("Invalid".to_owned())
    } else {
        Ok(())
    }
}

fn main() {
    let ashley = Custumer { age: 21 };
    let purchased = make_restricted(&ashley);
    println!("{:?}", purchased);
}
