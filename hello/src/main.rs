//fnIMPORTING HASH MAP
use std::collections::HashMap;

fn main() {
    //Declaring variables 
    println!("");
    println!("VARIABLES");
    println!("");
    let a = 10;
    let b = 15;
    println!("Hello, world, {} {}!", a,b);

    //Unsigned integer
    //u8, u16, u32, u64, u128
    let unsigned: u8 = 10;

    //Signed integer
    //i8, i16, i32, i64, i128
    let signed: i8 = -10;

    //float is used for decimals
    //f8, f16, f32, f64, f128
    let float: f32 = 1.2;

    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);

    //char - can only be
    let letter = "c";
    let emoji = "\u{1F600}"; // :D

    println!("letter: {}, emoji: {}", letter, emoji);

    let is_true: bool = true;

    println!("is_true: {}", is_true);

    //ARRAY
    println!("");
    println!("ARRAY");
    println!("");

    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    // print structure of array and other objects
    println!("{:?}", other_arr);

    //TUPLES
    println!("");
    println!("TUPLES");
    println!("");
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    //print structure of arrays and other objects
    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2 );

    let (a, b, c) = tuple;

    //destructuring
    println!("first {}, second {}, third {}", a, b, c);

    //FUNCTIONS
    println!("");
    println!("FUNCTIONS");
    println!("");
    print!("{}", is_even(2));

    //MUTABILITY
    println!("");
    println!("MUTABILITY");
    println!("");
    let mut number = 5;
    number = 3;
    println!("{}", number);

    //ARRAY 2
    println!("");
    println!("ARRAY");
    println!("");
    let array = [0, 1, 2, 3]; //length
    let slice = &arr[1 .. 3]; //[1, 2] don't know the length
    borrowing_slice(array, slice);

    //STRINGS
    // :: is used to reference a functions
    println!("");
    println!("STRINGS");
    println!("");
    let str: &str = "hello, world";
    let mut string: String = String::from("Hello World!");

    let slice = &string[.. 6];
    slice.len();

    string.push('1'); //allows yu to add a char
    string.push_str("! Bob"); //allows yu to add a string
    string = string.replace("Hello", "Bye");
    println!("{}", string);

    //if and else statements
    println!("");
    println!("IF AND ELSE STATEMENTS");
    println!("");
    let n = 3;
    if n > 0 {
        println!("{} is greater then 0", n);
    }  else if n < 0 {
        println!("{} is less then 0", n);
    } else {
        println!("is 0");
    }

    //for loop
    println!("");
    println!("FOR LOOP");
    println!("");
    for i in 0..6 {
        println!("{}", i);
    }

    //while loop
    println!("");
    println!("WHILE LOOP");
    println!("");

    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("EXIT");
            break
        }
    }

    //match statement
    println!("");
    println!("MATCH STATEMENT");
    println!("");

    let i = 5;
    match i {
        0 => println!("0"), // if the value is 0
        1 | 2 => println!("1, 2"), // if the value is 1 or 2
        3..=4 => println!("3, 4"), // if it is on range
        _=> println!("default") // if none of the cases match
    }

    //Struct
    println!("");
    println!("STRUCT");
    println!("");

    let name = String::from("Bird");
    let bird = Bird { name, attack: 5 };
    bird.print_name();
    println!("{} {}", bird.can_fly(), bird.is_animal());

    //ENUM
    println!("");
    println!("ENUM");
    println!("");

    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C{x:10, y:20};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(val) = b {
        println!("{:?}", val);
    }

    if let MyEnum::C{x, y} = c {
        println!("{} {}", x, y);
    }

    //VECTOR
    println!("");
    println!("VECTOR");
    println!("");

    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);

    //HASH MAPS
    println!("");
    println!("HASH MAPS");
    println!("");

    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "Hi2");
    println!("{:?}", map);

    //None, to indicate failure or lack of value, and
    //Some(value), a tuple struct that wraps a value with type T.

    match map.get(&0) {
        Some(str1) => println!("{}", str1),
        None=> println!("Doesn't exist in map"),
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        _=> println!("Doesn't exist in map"),
    }

    map.remove(&0);
    println!("{:?}", map);

    //OPTIONS
    println!("");
    println!("OPTIONS");
    println!("");

    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    //Unwrapping a 'Some' variant will extract the value wrapped.
    println!("{:?} unwraps to {}", divide1, divide2.unwrap());

    //Unwrapping a 'None' variant will 'panic!'
    //println!("{:?} unwraps to {}", divide1, divide2.unwrap());

    //RESULTS
    println!("");
    println!("RESULTS");
    println!("");

    let divided = divided(4, 2);
    //let res = divided.expect("we crashed");

    match divided {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v)
    }

    /* if divided.is_ok() {
        println!("{}", divided.unwrap());
    } */

    /* println!("{}", divided.unwrap()); */
    /* println!("{}", divided.unwrap_or(100)); */
    /* println!("{}", res); */

}












pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 //return bool
}

fn borrowing_slice(array: [u8; 4], slice: &[u8]) {
    println!("{:?}", array);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}

struct Bird {
    name: String,
    attack: u64
}

//impl to implement the struct
impl Bird {
    fn print_name(&self){ //&self is just a reference to the struct that you've created. Similar to the this function in javascript
        println!("{}", self.name);
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool{
        true
    }
}

//TRAIT
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

#[derive(Debug)]
enum MyError {
    Error1
}

//Err, to indicate failure or lack of value, and
//Ok(value), a tuple struct that wraps a value with type T.

fn divided(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}