use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
//todo: add comments
//todo: add tests
fn main() {
    //todo: "r_vocab" must search for a "text.txt" at its own level
    let mut file = File::open("src/text.txt").expect("<!> Can't open file!");
    let mut raw_vocab = String::new();

    file.read_to_string(&mut raw_vocab)
        .expect("<!> Can't read the file!");

    let re1 = Regex::new(r"[^A-Za-z]").unwrap();
    let re2 = Regex::new(r" {2,}").unwrap();

    raw_vocab = re1.replace_all(&mut raw_vocab, " ").to_string();
    raw_vocab = re2.replace_all(&mut raw_vocab, " ").trim().to_string();

    let mut raw_vocab_vec: Vec<String> = raw_vocab
        .split(" ")
        .map(|s| s.to_string().to_lowercase())
        .collect();

    raw_vocab_vec.sort();

    //main logic start

    let mut count = 0;
    let mut word = raw_vocab_vec.get(0).unwrap().to_string();
    let mut r_vocab_vec = vec![];

    for el in raw_vocab_vec {
        if word == el {
            count += 1;
        } else {
            r_vocab_vec.push(format!("{count}  {word}"));
            word = el;
            count = 1
        }
    }

    //todo: sorting must be like numbers, not like rows
    r_vocab_vec.sort();
    r_vocab_vec.reverse();

    //main logic end

    let r_vocab = r_vocab_vec.join("\n");
    //todo: "r_vocab" must create "r_vocab.txt" at its own level
    let mut file = File::create("src/r_vocab.txt").expect("<!> Can't create file!");

    file.write_all(r_vocab.as_bytes())
        .expect("<!> Can't write file!");
}
