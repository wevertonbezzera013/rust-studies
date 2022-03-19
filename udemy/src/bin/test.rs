fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}
#[cfg(test)]
mod tests {
    use create::*;

    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("hello");
        assert_eq!(result, expected, "strinng should be uppercase");
    }
}