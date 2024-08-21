use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::open("src/text.txt").expect("<!> Can't open file!");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("<!> Can't read the file!");

    let re = Regex::new(r"[^A-Za-z]").unwrap();
    let mut file = File::create("src/r_vocab.txt").expect("<!> Can't create file!");

    file.write_all(re.replace_all(&mut content, "\n").as_bytes())
        .expect("<!> Can't write file!");
}
