use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct BigramModel {
    token_counts: HashMap<String, i64>,
    bigram_counts: HashMap<(String, String), i64>
}

impl BigramModel {
    fn new() -> BigramModel {
        BigramModel {
            token_counts: HashMap::new(),
            bigram_counts: HashMap::new(),
        }
    }

    fn update_token_counts(
        &mut self,
        gram: String
    ) {
        match self.token_counts.get(&gram) {
            Some(count) => { self.token_counts.insert(gram, count + 1); }
            None => { self.token_counts.insert(gram, 1); }
        }
    }

    fn update_bigram_counts(
        &mut self,
        bigram: (String, String)
    ) {
        match self.bigram_counts.get(&bigram) {
            Some(count) => { self.bigram_counts.insert(bigram, count + 1); }
            None => { self.bigram_counts.insert(bigram, 1); }
        }
    }
}

fn main() {
    // Create an instance of the BigramModel
    let mut bmodel = BigramModel::new();

    // File text_sample.txt must exist in the current path
    if let Ok(lines) = read_lines("./text_sample.txt") {
        // Use lines from the iterator
        for line in lines.flatten() {
            if !line.is_empty() {
                //println!("{}", line);
                update_bigram_model(line, &mut bmodel)
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_bigram_model(line_of_text: String, _bmodel: &mut BigramModel) {
    let mut prev: &str = "<S>";
    // Take a line of text, and update the model with it 
    for gram in line_of_text.split_whitespace() {
        
        // Update token counts
        _bmodel.update_token_counts(gram.to_string());
        
        // Update bigram counts
        let bigram = (prev.to_string(), gram.to_string());
        _bmodel.update_bigram_counts(bigram);

        prev = gram;
        
    }

    // Add a end-of-sentence token, so the probabilities are cool
    _bmodel.update_bigram_counts((prev.to_string(), "</S>".to_string()))
}
