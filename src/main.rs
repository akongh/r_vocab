use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

//todo: add comments
fn main() {
    let mut file = File::open("src/text.txt").expect("<!> Can't open file!");
    let mut raw_vocab = String::new();

    file.read_to_string(&mut raw_vocab)
        .expect("<!> Can't read the file!");

    let re1 = Regex::new(r"[^A-Za-z]").unwrap();
    let re2 = Regex::new(r" {2,}").unwrap();

    let mut file = File::create("src/r_vocab.txt").expect("<!> Can't create file!");

    raw_vocab = re1.replace_all(&mut raw_vocab, " ").to_string();
    raw_vocab = re2.replace_all(&mut raw_vocab, " ").trim().to_string();

    let raw_vocab_vec: Vec<String> = raw_vocab
        .split(" ")
        .map(|s| s.to_string().to_lowercase())
        .collect();

    //todo: main logic

    let r_vocab = raw_vocab_vec.join("\n");

    file.write_all(r_vocab.as_bytes())
        .expect("<!> Can't write file!");
}
