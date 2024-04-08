use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct BigramModel {
    pub token_counts: HashMap<String, i64>,
    pub bigram_counts: HashMap<(String, String), i64>,
    start_of_sentence: String,
    end_of_sentence: String,
    sentence_tokens: Vec<String>
}

impl BigramModel {
    pub fn new() -> BigramModel {
        BigramModel {
            token_counts: HashMap::new(),
            bigram_counts: HashMap::new(),
            start_of_sentence: "<S>".to_string(),
            end_of_sentence: "</S>".to_string(),
            sentence_tokens: vec!["<S>".to_string(), "</S>".to_string()]
        }
    }

    fn get_token_count(
        &mut self,
        gram: String
    ) -> i64 {
        match self.token_counts.get(&gram) {
            Some(count) => { return *count; }
            None => { return 0; }
        }
    }

    fn get_bigram_count(
        &mut self,
        bigram: (String, String)
    ) -> i64 {
        match self.bigram_counts.get(&bigram) {
            Some(count) => { return *count; }
            None => { return 0; }
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

    pub fn calculate_bigram_probability(
        &mut self,
        bigram: &(String, String)
    ) -> f64 {
        let bigram_count = self.get_bigram_count(bigram.clone());
        let token_count = self.get_token_count(bigram.0.to_string());
    
        if token_count.eq(&0) {
            // Catch a divide by zero to stop it returning NaN
            return 0 as f64;
        } else {
            return bigram_count as f64 / token_count as f64;
        }    
    }

    pub fn update_bigram_model(
        &mut self,
        line_of_text: String
    ) {
        let mut prev: &str = &self.start_of_sentence.to_string();
        // Take a line of text, and update the model with it 
        for gram in line_of_text.split_whitespace() {
            // Update token counts
            self.update_token_counts(gram.to_string());
            
            // Update bigram counts
            let bigram = (prev.to_string(), gram.to_string());
            self.update_bigram_counts(bigram);
    
            prev = gram;
        }
    
        // Add a end-of-sentence token, so the probabilities are cool
        self.update_bigram_counts((prev.to_string(), self.end_of_sentence.to_string()))
    }

    pub fn most_common_bigram(
        &mut self
    ) -> Result<(&(String, String), &i64), &str> {
        return self.bigram_counts
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .ok_or("Couldn't find a bigram");
    }

    pub fn most_common_bigram_without_sentence_tokens(
        &mut self
    ) -> Result<(&(String, String), &i64), &str> {
        return self.bigram_counts
            .iter()
            .filter(|a| !self.sentence_tokens.contains(&a.0.0) && !self.sentence_tokens.contains(&a.0.1))
            .max_by(|a, b| a.1.cmp(&b.1))
            .ok_or("Couldn't find a bigram");
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
