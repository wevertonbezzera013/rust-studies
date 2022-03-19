// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    locker: Option<i32>,
    name: Option<String>,
}

fn main() {
    let response = Student {
        locker: Some(32),
        name: Some("Billy".to_owned())
    };

    match response.locker {
        Some(std) => println!("Locker: {:?}", std),
        None => println!("Wrong password"),
    }

    match response.name {
        Some(std) => println!("Name: {:?}", std),
        None => println!("Wrong name"),
    }
}
