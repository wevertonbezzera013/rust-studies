fn main() {
    let mut data = Some(3);
    while let Some(i) = data {
        println!("loop");
        data = None;
    }
    let numbers = vec![1, 2, 3, 4, 5];
    let mut number_iter = numbers.iter();
    while let Some(num) = number_iter.next() {
        println!("{:?}", num);
    }

    println!("Done!")
}