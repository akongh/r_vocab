use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::time::Instant;
//todo: add comments
//todo: add tests
//todo: add colors for console text

fn main() {
    let now = Instant::now();

    let mut file = File::open("text.txt").expect(">>>>>>>> Can't open file!\n");
    let mut raw_vocab = String::new();
    file.read_to_string(&mut raw_vocab)
        .expect(">>>>>>>> Can't read the file!\n");
    if raw_vocab.len() == 0 {
        println!("> Source file is empty.");
        println!("> NOT DONE!");
        process::exit(0);
    }
    println!("> The text has been obtained from a source file.");

    let re1 = Regex::new(r"[^A-Za-z]").unwrap();
    let re2 = Regex::new(r" {2,}").unwrap();
    raw_vocab = re1.replace_all(&mut raw_vocab, " ").to_string();
    raw_vocab = re2.replace_all(&mut raw_vocab, " ").trim().to_string();
    if raw_vocab.len() == 0 {
        println!("> The source file does not have English words.");
        println!("> NOT DONE!");
        process::exit(0);
    }

    let mut raw_vocab_vec: Vec<String> = raw_vocab
        .split(" ")
        .map(|s| s.to_string().to_lowercase())
        .collect();

    raw_vocab_vec.sort();

    let mut raw_vocab_vec_clear = vec![];
    let min_word_long = 2;
    for el in raw_vocab_vec {
        if el.len() >= min_word_long {
            raw_vocab_vec_clear.push(el);
        }
    }
    println!("> The source text has been cleared.");
    if raw_vocab_vec_clear.len() == 0 {
        println!("> The source file does not have English words with a length of {min_word_long} or more letters.");
        println!("> NOT DONE!");
        process::exit(0);
    }

    //main logic start

    let mut count = 0;
    let mut word = raw_vocab_vec_clear.get(0).unwrap().to_string();
    let mut r_vocab_vec = vec![];
    for el in raw_vocab_vec_clear {
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
        let el0 = el.0.to_string();
        let el1 = el.1;
        r_vocab_vec_string.push(format!("{el0}  {el1}"));
    }
    println!("> The vocabulary has been created.");

    //main logic end

    let count = r_vocab_vec_string.len();
    let title = "Total unique words: ".to_string() + &count.to_string() + "\n";
    let mut r_vocab_vec_full = vec![];
    r_vocab_vec_full.push(title);
    r_vocab_vec_full.append(&mut r_vocab_vec_string);

    let r_vocab = r_vocab_vec_full.join("\n");
    let mut file = File::create("r_vocab.txt").expect(">>>>>>>> Can't create file!\n");
    file.write_all(r_vocab.as_bytes())
        .expect(">>>>>>>> Can't write file!\n");
    println!("> The vocabulary has been written in a file.");

    let elapsed = now.elapsed();

    println!("> DONE!");
    println!("Total time: {:.2?}", elapsed)
}
