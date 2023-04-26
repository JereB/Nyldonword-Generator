use std::{cmp::Ordering, collections::BTreeSet};
use rayon::prelude::*;

struct NyldonWord {
    pub w : String,
    pub p  : String,
    pub s : String,
}


impl Ord for NyldonWord {
    fn cmp(&self, other: &NyldonWord) -> Ordering {
        match self.w.len().cmp(&other.w.len()) {
            Ordering::Equal => self.w.cmp(&other.w),
            ord => ord,
        } 
    }
}

impl PartialOrd for NyldonWord {
    fn partial_cmp(&self, other: &NyldonWord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NyldonWord {
    fn eq(&self, other: &NyldonWord) -> bool {
        self.w == other.w
    }
}

impl Eq for NyldonWord {}


pub struct WordGenerator {
    current: usize,
    max: usize,
}

impl WordGenerator {
    pub fn new(length: usize) -> WordGenerator {
        WordGenerator {
            current: 1 << (length - 1),
            max : 1 << length, 
        }
    }
}


impl Iterator for WordGenerator {
    type Item = String;

    // return the next binary word
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }
        let word = format!("{:b}", self.current);
        self.current += 1;
        Some(word)
    }
}



fn main() {
    
    let mut words = BTreeSet::new();
    let starttime = std::time::Instant::now();
    init_words(&mut words);

    for i in 2..=200 {

        let iter_starttime = std::time::Instant::now();
        let word_generator = WordGenerator::new(i);

        let new_nyldon = word_generator
            .par_bridge()
            .filter_map(|word| find_nyldon_factors(word, &words))
            .collect::<Vec<_>>();

        new_nyldon.iter().for_each(|nyldon_word| {
           println!("{},{},{}", nyldon_word.w, nyldon_word.p, nyldon_word.s)
        });

        eprintln!("{}: {}", i, new_nyldon.len());

        words.extend(new_nyldon);

        eprintln!("{}: {}s", i, iter_starttime.elapsed().as_secs_f64());
    }

    eprintln!("{}s", starttime.elapsed().as_secs_f64());

}


fn find_nyldon_factors(word: String, nyldon_words: &BTreeSet<NyldonWord>) -> Option<NyldonWord>{

     
    let opt_s = nyldon_words.iter()
        .rev()
        .find(|&nyldon_word| {
            word.ends_with(&nyldon_word.w)
        });
    
    if let Some(s) = opt_s {
        //part of word that is before p
        let p = &word[0..word.len() - s.w.len()];

        // if p > s.w
        if p > &s.w && nyldon_words.iter().any(|nyldon_word| nyldon_word.w == p) {
            Some(NyldonWord {
                w: word.to_string(),
                p: p.to_string(),
                s: s.w.clone(),
            })
        } else {
            None
        }
    } else {
        None
    }
}




fn init_words(words: &mut BTreeSet<NyldonWord>) {
    words.insert(NyldonWord {
        w: "0".to_string(),
        p: "".to_string(),
        s: "".to_string(),
    });

    words.insert(NyldonWord {
        w: "1".to_string(),
        p: "".to_string(),
        s: "".to_string(),
    });
}


