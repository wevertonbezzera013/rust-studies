// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String
}

fn main() {
    let people = vec![
        Person {
            age: 21,
            name: "Weverton".to_owned(),
            color: String::from("black"),
        },
        Person {
            age: 18,
            name: String::from("Wong"),
            color: String::from("red"),
        },
        Person {
            age: 22,
            name: String::from("Tom"),
            color: String::from("White"),
        },
    ];

    for society in people {
        if society.age > 10 {
            println!("Name: {:?}, Age: {:?}, Color: {:?}", society.name, society.age, society.color);
        } else {
            println!("No")
        }
    }
}
