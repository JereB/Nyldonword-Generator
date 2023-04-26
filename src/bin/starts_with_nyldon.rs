use std::{io::Read, fs::File};

use rayon::prelude::*;



fn main() {
    let nyldon = read_nyldon();

    let start_time = std::time::Instant::now();

    nyldon.iter()
        .par_bridge()
        .filter(|word| !word_equals_repeating_nyldon(word, &nyldon))
        .for_each(|word| println!("{}", word));

    eprintln!("{}s", start_time.elapsed().as_secs_f64());
}



fn word_equals_repeating_nyldon(prefix: &str, nyldon: &[String]) -> bool {
    for prefix in all_prefixes(prefix) {
        
        let prefix_equals_repeating_nyldon = nyldon.iter()
            .filter(|nyldon| nyldon.len() <= prefix.len())
            .any(|nyldon_word| equals_repeating(nyldon_word, &prefix));

        if !prefix_equals_repeating_nyldon {
            return false;
        }
    }

    true
}



fn equals_repeating(repeating_word: &str, prefix: &str) -> bool {

    if repeating_word.len() * 2 -1 < prefix.len() {
        return false;
    }

    //repeat the iterator of the characters of the word
    let doubled_word = repeating_word.to_string() + &repeating_word[..repeating_word.len()-1];
    let mut word_iter = doubled_word.chars();
    let mut prefix_iter = prefix.chars();

    // check if the characters of the prefix are equal to the characters of the word
    for _ in 0..prefix.len() {
        if word_iter.next().unwrap() != prefix_iter.next().unwrap() {
            return false;
        }
    }

    true
}

fn all_prefixes(word: &str) -> Vec<String> {
    let mut prefixes = Vec::new();

    for i in 1..word.len() {
        prefixes.push(word[0..i].to_string());
    }

    prefixes
}


fn read_nyldon() -> Vec<String> {
    let mut nyldon = Vec::new();

    // read content of file in string
    let mut file = File::open("nyldon.csv").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // take each line to first comma except the first line
    for line in content.lines() {
        let word = line.split(',').next().unwrap();
        nyldon.push(word.to_string());
    }
    
    nyldon
}