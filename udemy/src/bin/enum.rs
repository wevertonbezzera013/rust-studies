enum Direction {
    Left,
    Right
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("Go Left"),
        Direction::Right => println!("Go Right"),
    }
}