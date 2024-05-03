use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

use select::document::Document;
use select::predicate::Name;
use scraper::{Html, Selector};
use reqwest;

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

    // TODO: Method for loading and saving (to disk) relevant data structures?

    // TODO: Calculate perplexity?

    fn get_last_given_penultimate_count(
        last_given_penultimate_counts: &HashMap<Vec<String>, HashMap<String, i64>>,
        pen_gram: Vec<String>,
        last: String
    ) -> i64 {
        match last_given_penultimate_counts.get(&pen_gram) {
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
        penultimate_gram_counts: &HashMap<Vec<String>, i64>,
        pen_gram: Vec<String>
    ) -> i64 {
        match penultimate_gram_counts.get(&pen_gram) {
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
        penultimate_gram_counts: &HashMap<Vec<String>, i64>,
        last_given_penultimate_counts: &HashMap<Vec<String>, HashMap<String, i64>>,
        ngram: &Vec<String>
    ) -> f64 {
        // To Calculate:
        // For n_gram [A,B,C]
        // Given n_gram[0:len-1], what is the probability of n_gram[len]
        // For n_gram[0:len-1] grams, we need to count how many times these occur (the denominator)
        // Then we need to store, for each n_gram[0:len-1], how many times each n_gram[len] occurs (numberator)
        if let Some((last, penultimate_gram)) = ngram.split_last() {
            let last_given_penultimate_count = NGramModel::get_last_given_penultimate_count(last_given_penultimate_counts, penultimate_gram.to_vec(), last.to_string());
            let penultimate_gram_count = NGramModel::get_penultimate_count(penultimate_gram_counts, penultimate_gram.to_vec());
            
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

    // TODO: Look into smoothing - 'backoff', 'interpolation', 'laplace smoothing' 

    pub fn probability_of_sentence(
        &mut self,
        line_of_text: String
    ) -> f64 {
        // Calculation equations
        // Unigram:  ( 1 )
        //  P(w1, . . . wn) = PROD^{n}_{i=1} P(wi)
        // Bigram:   ( 2 )
        //  P(w1, . . . wn) = P(w1) PROD^{n}_{i=2} P(wi|wi−1)
        // Trigram:  ( 3 )
        //  P(w1, . . . wn) = P(w1)P(w2|w1) PROD^{n}_{i=3} P(wi|wi−2, wi−1)
        // Quadgram: ( 4 )
        //  P(w1, . . . wn) = P(w1)P(w2|w1)P(w3|w2, w1) PROD^{n}_{i=4} P(wi|wi-3, wi−2, wi−1)

        let words: Vec<String> = self.string_to_string_vec(line_of_text);
        if words.len() < self.degree.try_into().unwrap() {
            return self.probability_for_partial_ngram(&words);
        } else if words.len() == self.degree.try_into().unwrap() {
            return NGramModel::calculate_ngram_probability(
                &self.penultimate_gram_counts,
                &self.last_given_penultimate_counts,
                &words
            );
        } else {
            // words.len() > self.degree
            // Sum log probabilities at this stage, so as to not incur small floating point number errors

            // Populate a vector of all the probabilities
            let mut probabilities: Vec<f64> = vec![];
            if self.degree != 1 {
                // Do the first part of the calculation
                for i in 1..self.degree {
                    let current_size = i.try_into().unwrap();
                    // Store log2 of probability
                    probabilities.push(
                        self.probability_for_partial_ngram(
                            &words[0..current_size].to_vec()
                        ).log2()
                    );
                }
            } 
            
            // We need to skip the first section
            let words_start_point = (self.degree - 1).try_into().unwrap();
            for grams in words[words_start_point..].windows(self.degree.try_into().unwrap()) {
                // Store log2 of probability
                probabilities.push(
                    NGramModel::calculate_ngram_probability(
                        &self.penultimate_gram_counts,
                        &self.last_given_penultimate_counts,
                        &grams.to_vec()
                    ).log2()
                );
            }

            // Sum log probs, then exponent to retrieve the underlying number
            let sum_of_log_probs: f64 = probabilities
                .iter()
                .sum();
            return sum_of_log_probs.exp2();
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

    pub fn generate_text(
        &mut self,
        generate_mode: String,
        number_of_sentences: i32
    ) -> Vec<String> {
        if !["Probabilistic", "Greedy"].contains(&&*generate_mode) {
            panic!("Unrecognised generate_mode supplied: {:?}", generate_mode);
        }

        let mut generated_sentences: Vec<String> = vec![];
        for _i in 0..number_of_sentences {
            generated_sentences.push(
                self.generate_text_individual_sentence(&generate_mode)
            )
        }
        return generated_sentences;
    }

    fn generate_text_individual_sentence(
        &mut self,
        generate_mode: &String
    ) -> String {
        let max_sentence_size = 25;
        // This will be greedy_based
        let mut history: Vec<String> = vec![];
        // Initialise history as (degree - 1) start tokens
        for _i in 0..(self.degree - 1).try_into().unwrap() {
            history.push(self.start_of_sentence.to_string());
        }

        let mut generated_grams_storage: Vec<String> = vec![];

        let mut generated_gram: String = "".to_string();
        while generated_gram != self.end_of_sentence &&
            generated_grams_storage.len() < max_sentence_size {
            // Keep generating grams based on the history
            if generate_mode == "Probabilistic" {
                generated_gram = self.get_most_frequent_gram_prob(&history);
            } else if generate_mode == "Greedy" {
                generated_gram = self.get_most_frequent_gram(&history);
            } else {
                panic!("Unrecognised generate_mode supplied: {:?}", generate_mode);
            }
            
            // Remove and rotate history, only if history is large enough
            if self.degree > 1 {
                history.remove(0);
                history.push(generated_gram.clone().to_string());
            }

            // Track the generated_gram
            generated_grams_storage.push(generated_gram.clone().to_string())
        }

        if generated_grams_storage.last().unwrap().to_string() == self.end_of_sentence {
            return generated_grams_storage[0..generated_grams_storage.len() - 1].join(" ");
        } else {
            return generated_grams_storage.join(" ");
        }
    }

    fn get_most_frequent_gram_prob (
        &mut self,
        history: &Vec<String>
    ) -> String {
        // probabilistic, random sampling, the biased dice thing
        let history_size = history.len();
        assert!(history_size == (self.degree - 1).try_into().unwrap());

        let mut rng = rand::thread_rng();
        let rand_value = rng.gen::<f64>();

        let mut accumulated_prob: f64 = 0.0;

        for (gram, _k) in &self.ngram_counts {
            // Shortcircuit for history equals 0
            if history_size == 0 || gram.to_vec()[0..history_size] == history.to_vec() {
                accumulated_prob += NGramModel::calculate_ngram_probability(
                    &self.penultimate_gram_counts,
                    &self.last_given_penultimate_counts,
                    &gram
                );
                if accumulated_prob >= rand_value {
                    return gram.last().unwrap().to_string();
                }
            }
        }

        // TODO: This is 1.0000000000000013 - is this okay?
        // Sum of suitable ngrams - should be 1
        // let sum_value: f64 = suitable_ngrams
        //     .iter()
        //     .map(|f| self.calculate_ngram_probability(f.0))
        //     .sum();

        // We should never get here
        // Return the first from the keys
        return "".to_string();
    }

    fn get_most_frequent_gram (
        &mut self,
        history: &Vec<String>
    ) -> String {
        let history_size = history.len();
        assert!(history_size == (self.degree - 1).try_into().unwrap());

        let mut tracking_max = -1;
        let mut maximum_end_ngrams: Vec<String> = vec![];

        // Iterate once through ngram_counts to populate maximum_end_ngrams
        for (gram, k) in &self.ngram_counts {
            // Shortcircuit for history equals 0
            if history_size == 0 || gram.to_vec()[0..history_size] == history.to_vec() {
                // Check if k is at tracking max
                if k < &tracking_max {
                    // Do nothing
                    {}
                } else if *k == tracking_max {
                    // Add to maximum_end_ngrams
                    maximum_end_ngrams.push(gram.last().unwrap().to_string())
                } else {
                    // k > tracking_max
                    tracking_max = *k;
                    maximum_end_ngrams.clear();
                    maximum_end_ngrams.push(gram.last().unwrap().to_string())
                }
            }
        }
        
        // Sort alphabetically
        maximum_end_ngrams.sort();
        let most_frequent_gram: String = maximum_end_ngrams.first().unwrap().to_string();

        return most_frequent_gram;
    }

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

pub struct SOTUScraper {
    text_lines: Vec<String>
}

impl SOTUScraper {
    pub fn gather_text() -> SOTUScraper {
        // Goal: gather lines of text from 
        let mut sotu_lines: Vec<String> = vec![];

        let directory_url = "https://www.presidency.ucsb.edu/documents/presidential-documents-archive-guidebook/annual-messages-congress-the-state-the-union";
        let text = SOTUScraper::get_text_from_url(directory_url);

        println!("Downloading SOTU links from UCSB.");

        let mut sotu_links: Vec<String> = vec![];

        match text {
            Ok(content) => {
                // Find start of tables
                let mut table_contents: Vec<String> = vec![];
                
                Document::from(content.as_str())
                    .find(Name("table"))
                    .for_each(|x| table_contents.push(x.html()));

                for individual_table in table_contents {
                    Document::from(individual_table.as_str())
                        .find(Name("a"))
                        .filter_map(|n| n.attr("href"))
                        .filter(|a| a.contains("https://www.presidency.ucsb.edu/documents/"))
                        .for_each(|x| sotu_links.push(x.to_string()));
                }
            }
            Err(e) => panic!("Failed to get text: {e:?}")
        }

        sotu_links.sort_unstable();
        sotu_links.dedup();
        let total_links = sotu_links.len();
        println!("We have gathered {:?} SOTU links.", total_links);

        // Build selector for SOTU content
        let selector = Selector::parse(r#"div[class="field-docs-content"]"#).unwrap();

        for (pos, sotu_link) in sotu_links.iter().enumerate() {
            println!("{:?}/{total_links} -- Downloading: '{sotu_link}'", pos+1);
            let sotu_text = SOTUScraper::get_text_from_url(&sotu_link);
            match sotu_text {
                Ok(content) => {
                    let fragment = Html::parse_fragment(&content);
                    let ul = fragment.select(&selector).next().unwrap();
                    let text_lines = ul.child_elements().flat_map(|el| el.text()).collect::<Vec<_>>();
                    
                    // Got the content and loading it into the model
                    for line in text_lines {
                        for part_line in line.split(".") {
                            if !part_line.is_empty() {
                                sotu_lines.push(part_line.to_string());
                            }
                        }
                    }
                }
                Err(e) => panic!("Failed to get text: {e:?}")
            }
        }
        
        return SOTUScraper {text_lines: sotu_lines}
    }

    fn get_text_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let http_client = reqwest::blocking::Client::new();
        
        let response = http_client
            // form a get request with get(url)
            .get(url)
            // send the request and get Response or else return the error
            .send()?
            // get text from response or else return the error
            .text()?;

        // wrapped response in Result
        Ok(response)
    }

    pub fn get_line_iterator(
        &mut self
    ) -> std::slice::Iter<'_, String> {
        return self.text_lines.iter();
    }

}
