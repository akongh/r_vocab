use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
//todo: add comments
//todo: add tests
fn main() {
    let mut file = File::open("text.txt").expect("<!> Can't open file!\n");
    let mut raw_vocab = String::new();

    file.read_to_string(&mut raw_vocab)
        .expect("<!> Can't read the file!\n");

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
            let count_word = (count, word);
            r_vocab_vec.push(count_word);
            word = el;
            count = 1
        }
    }

    r_vocab_vec.sort_by(|a, b| b.0.cmp(&a.0));

    let mut r_vocab_vec_string = vec![];

    for el in r_vocab_vec {
        // format!("{count}  {word}")
        let el0 = el.0.to_string();
        let el1 = el.1;
        r_vocab_vec_string.push(format!("{el0}  {el1}"));
    }

    //main logic end

    let r_vocab = r_vocab_vec_string.join("\n");
    let mut file = File::create("r_vocab.txt").expect("<!> Can't create file!\n");

    file.write_all(r_vocab.as_bytes())
        .expect("<!> Can't write file!\n");
}
