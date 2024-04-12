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
    // TODO: Sum log probs, in and out
    // Unigram:  ( 1 )
    //  P(w1, . . . wn) = PROD^{n}_{i=1} P(wi)
    // Bigram:   ( 2 )
    //  P(w1, . . . wn) = P(w1) PROD^{n}_{i=2} P(wi|wi−1)
    // Trigram:  ( 3 )
    //  P(w1, . . . wn) = P(w1)P(w2|w1) PROD^{n}_{i=3} P(wi|wi−2, wi−1)
    // Quadgram: ( 4 )
    //  P(w1, . . . wn) = P(w1)P(w2|w1)P(w3|w2, w1) PROD^{n}_{i=4} P(wi|wi-3, wi−2, wi−1)
    //
    // P(w1), for Bigram, means all the grams that start with w1, divided by count of all
    //
    pub fn probability_of_sentence(
        &mut self,
        line_of_text: String
    ) -> f64 {
        let words: Vec<String> = self.string_to_string_vec(line_of_text);
        if words.len() < self.degree.try_into().unwrap() {
            return self.probability_for_partial_ngram(&words);
        } else if words.len() == self.degree.try_into().unwrap() {
            return self.calculate_ngram_probability(&words)
        } else {
            // words.len() > self.degree
            // TODO: Follow paradigm above
            return 0.0;
        }
    }

    fn count_of_partial_ngram(
        &mut self,
        partial_gram: &Vec<String>
    ) -> i64 {
        let partial_size = partial_gram.len();
        // Filter ngram_counts for the keys, sliced to partial_size, equal to partial_gram
        // Sum the values and return
        return self.ngram_counts
            .iter()
            .filter(|a|
                    a.0.to_vec()[0..partial_size] == partial_gram.to_vec()
            ).map(|(_a, b)| b).sum();
    }

    pub fn probability_for_partial_ngram(
        &mut self,
        partial_gram: &Vec<String>
    ) -> f64 {
        assert!(partial_gram.len() < self.degree.try_into().unwrap());
        if partial_gram.len() == 1 {
            // Divide by total number of ngrams
            let total_number_of_ngrams: i64 = self.ngram_counts.values().sum();
            return self.count_of_partial_ngram(partial_gram) as f64 / total_number_of_ngrams as f64;
        } else {
            // P(w2|w1) = count(gram[w1, w2]) / count(gram[w1])
            // P(w3|w2, w1) = count(gram[w1, w2, w3]) / count(gram[w1, w2])
            let context_partial_gram = &partial_gram[0..partial_gram.len() - 1].to_vec();
            return self.count_of_partial_ngram(partial_gram) as f64 / self.count_of_partial_ngram(context_partial_gram) as f64;
        }
    }

    fn string_to_string_vec(
        &mut self,
        line_of_text: String
    ) -> Vec<String>{
        let cleaned_text = line_of_text.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '-', '!', '?', '"', '[', ']', '/', '\\'][..], "");
        return cleaned_text.split_whitespace().map(str::to_string).collect();
    }


    // TODO: generate_text
    //       method: most frequent (greedy)
    //               probabilistic, random sampling, the biased dice thing

    pub fn update_ngram_model(
        &mut self,
        line_of_text: String
    ) {
        let mut words: Vec<String> = self.string_to_string_vec(line_of_text);
        
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
