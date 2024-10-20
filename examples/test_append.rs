use std::fs::read_to_string;

fn main() {
    let resp = reqwest::blocking::get("https://gist.githubusercontent.com/Dumpinground/5f10045365df9e0c6406308675a01157/raw/b564cb155e89bf7fd7028284e3dcdfd190b65d82/text.txt").expect("request failed");
    let text = resp.text().expect("body invalid");
    // let text = read_to_string("text.txt").unwrap();
    println!("{}", text);
    let mut result = String::new();
    for t in text.chars() {
        if t == '\n' {
            result += "}'";
        }
        result += &String::from(t);
        println!("{result}");
    }
}
