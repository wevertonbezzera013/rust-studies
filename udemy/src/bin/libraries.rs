fn main() {
    let numbers = vec![1, 2, 3, 4];
    match numbers.is_empty() {
        true => println!("No numbers"),
        false => println!("{:?}", numbers),
    }
}