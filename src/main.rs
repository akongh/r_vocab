use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("src/text.txt").expect("<!> Can't open file!");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("<!> Can't read the file!");

    let re1 = Regex::new(r"[^A-Za-z]").unwrap();
    let re2 = Regex::new(r" {2}").unwrap();

    let mut file = File::create("src/r_vocab.txt").expect("<!> Can't create file!");

    let mut result1 = re1.replace_all(&mut content, " ").to_string();
    let result2 = re2.replace_all(&mut result1, " ").to_string();
    let result = result2.trim().as_bytes();

    file.write_all(result).expect("<!> Can't write file!");
}
