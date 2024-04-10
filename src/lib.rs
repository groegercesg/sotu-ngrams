use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct NGramModel {
    pub last_given_penultimate_counts: HashMap<Vec<String>, HashMap<String, i64>>,
    pub penultimate_gram_counts: HashMap<Vec<String>, i64>,
    pub ngram_counts: HashMap<Vec<String>, i64>,
    degree: i64,
    start_of_sentence: String,
    end_of_sentence: String,
    sentence_tokens: Vec<String>
}

impl NGramModel {
    pub fn new(
        degree: i64
    ) -> NGramModel {
        NGramModel {
            last_given_penultimate_counts: HashMap::new(),
            penultimate_gram_counts: HashMap::new(),
            ngram_counts: HashMap::new(),
            degree: degree,
            start_of_sentence: "<S>".to_string(),
            end_of_sentence: "</S>".to_string(),
            sentence_tokens: vec!["<S>".to_string(), "</S>".to_string()]
        }
    }

    fn get_last_given_penultimate_count(
        &mut self,
        pen_gram: Vec<String>,
        last: String
    ) -> i64 {
        match self.last_given_penultimate_counts.get(&pen_gram) {
            Some(pen_gram_map) => { 
                match pen_gram_map.get(&last) {
                    Some(count) => { return *count; }
                    None => { return 0;}
                }
            }
            None => { return 0; }
        }
    }

    fn get_penultimate_count(
        &mut self,
        pen_gram: Vec<String>
    ) -> i64 {
        match self.penultimate_gram_counts.get(&pen_gram) {
            Some(count) => { return *count; }
            None => { return 0; }
        }
    }

    fn update_last_given_penultimate_counts(
        &mut self,
        pen_gram: &Vec<String>,
        last: &String
    ) {
        match self.last_given_penultimate_counts.get_mut(pen_gram) {
            Some(pen_gram_map) => {
                match pen_gram_map.get(last) {
                    Some(count) => { 
                        pen_gram_map.insert(last.to_string(), *count + 1);
                    }
                    None => { pen_gram_map.insert(last.to_string(), 1); }
                }
            }
            None => { 
                let mut pen_gram_map = HashMap::new();
                pen_gram_map.insert(last.to_string(), 1);
                self.last_given_penultimate_counts.insert(pen_gram.to_vec(), pen_gram_map);
            }
        }
    }

    fn update_penultimate_counts(
        &mut self,
        pen_gram: &Vec<String>
    ) {
        match self.penultimate_gram_counts.get(pen_gram) {
            Some(count) => { self.penultimate_gram_counts.insert(pen_gram.to_vec(), count + 1); }
            None => { self.penultimate_gram_counts.insert(pen_gram.to_vec(), 1); }
        }
    }

    fn update_ngram_counts(
        &mut self,
        ngram: &Vec<String>
    ) {
        match self.ngram_counts.get(ngram) {
            Some(count) => { self.ngram_counts.insert(ngram.to_vec(), count + 1); }
            None => { self.ngram_counts.insert(ngram.to_vec(), 1); }
        }
    }

    pub fn calculate_ngram_probability(
        &mut self,
        ngram: &Vec<String>
    ) -> f64 {
        // To Calculate:
        // For n_gram [A,B,C]
        // Given n_gram[0:len-1], what is the probability of n_gram[len]
        // For n_gram[0:len-1] grams, we need to count how many times these occur (the denominator)
        // Then we need to store, for each n_gram[0:len-1], how many times each n_gram[len] occurs (numberator)
        if let Some((last, penultimate_gram)) = ngram.split_last() {
            let last_given_penultimate_count = self.get_last_given_penultimate_count(penultimate_gram.to_vec(), last.to_string());
            let penultimate_gram_count = self.get_penultimate_count(penultimate_gram.to_vec());
            
            if penultimate_gram_count.eq(&0) {
                // Catch a divide by zero to stop it returning NaN
                return 0 as f64;
            } else {
                return last_given_penultimate_count as f64 / penultimate_gram_count as f64;
            }   
        } else {
            panic!("Split last_mut failed");
        }
    }

    // TODO: Probability_of_sentence
    //       P(w1) * PROD P(Wn | Wn-1)
    //       Sum log probs, in and out


    // TODO: generate_text
    //       method: most frequent (greedy)
    //               probabilistic, random sampling, the biased dice thing

    pub fn update_ngram_model(
        &mut self,
        line_of_text: String
    ) {
        let cleaned_text = line_of_text.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '-', '!', '?', '"', '[', ']', '/', '\\'][..], "");

        let mut words: Vec<String> = cleaned_text.split_whitespace().map(str::to_string).collect();
        
        // Add (degree - 1) start and end tokens to the words
        for _i in 0..(self.degree - 1) {
            words.insert(0, self.start_of_sentence.to_string());
            words.push(self.end_of_sentence.to_string());
        }
        
        // Take a line of text, and update the model with it 
        for ngram in words.windows(self.degree.try_into().unwrap()) {
            assert!(ngram.len() == self.degree.try_into().unwrap());

            let last = ngram.split_last().unwrap().0;
            let penultimate_gram = ngram.split_last().unwrap().1.to_vec();
            
            // Update last_given_penultimate counts
            self.update_last_given_penultimate_counts(&penultimate_gram, last);
            // Update penultimate counts
            self.update_penultimate_counts(&penultimate_gram);
            // Update ngram counts
            self.update_ngram_counts(&ngram.to_vec());
        }
    }

    pub fn most_common_ngram(
        &mut self
    ) -> Result<(&Vec<String>, &i64), &str> {
        return self.ngram_counts
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .ok_or("Couldn't find a bigram");
    }

    pub fn most_common_ngram_without_sentence_tokens(
        &mut self
    ) -> Result<(&Vec<String>, &i64), &str> {
        return self.ngram_counts
            .iter()
            // Have to iter over all elements of the vector, checking they're not in self.sentence_tokens
            .filter(|a| { 
                    a.0.to_vec().iter()
                    .filter(|gram| self.sentence_tokens.contains(gram))
                    .count() == 0
                }
            )
            .max_by(|a, b| a.1.cmp(&b.1))
            .ok_or("Couldn't find a bigram");
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
