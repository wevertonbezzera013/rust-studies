fn main() {
    let my_name = "tom";
    match my_name {
        "wevertom" => println!("that is my name"),
        "tom" => println!("that is NOT my name"),
        "Henry" => println!("hello"),
        _ => println!("it isn't anything else!"),
    }
}