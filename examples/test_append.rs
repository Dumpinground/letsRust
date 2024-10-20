use std::fs::read_to_string;

fn main() {
    let text = read_to_string("text.txt").unwrap();
    let mut result = String::new();
    for t in text.chars() {
        if t == '\n' {
            result += "}'";
        }
        result += &String::from(t);
        println!("{result}");
    }
}
