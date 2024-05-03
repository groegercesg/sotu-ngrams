use std::env;

use grams::NGramModel;
use grams::SOTUScraper;

fn main() {
    // Get program arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough program arguments supplied, you supplied: {:?}", args);
    }

    // Create an instance of the NGramModel
    let mut ngmodel = NGramModel::new(4);

    let mut sotu_scraper = SOTUScraper::gather_text();

    for sotu_line in sotu_scraper.get_line_iterator() {
        ngmodel.update_ngram_model(sotu_line.to_string());
    }

    let most_common_ngram_result = ngmodel.most_common_ngram_without_sentence_tokens();
    
    assert!(most_common_ngram_result.is_ok());
    println!("The most frequent ngram was: {:?}. It occurred {:?} times.", 
        most_common_ngram_result.unwrap().0,
        most_common_ngram_result.unwrap().1
    );

    // Generate 10 sample sentences
    println!("I generated some sample sentences for you:");
    for generated_sentence in ngmodel.generate_text("Probabilistic".to_string(), 10) {
        println!("\t{:?}.",
            generated_sentence
        );
    }
}
