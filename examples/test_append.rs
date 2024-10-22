use std::fs::read_to_string;

fn main() {
    let text = read_to_string("text.txt").unwrap();
    append_postfix(&text);
}

fn append_postfix(text: &str) -> String {
    let mut result = String::new();
    let s = "\r\n-";
    for t in text.chars() {
        if s.contains(t) {
            result += "↙";
        }
        result += &String::from(t);
        println!("{}", result.lines().last().unwrap());
    }
    result
}

#[test]
fn text_compare() {
    let text1 = read_to_string("text.txt").unwrap();
    let text2 = "state = :state
state - a = :state
";
    assert_eq!(text1, text2);
}
