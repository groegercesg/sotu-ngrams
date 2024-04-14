use std::env;

use grams::NGramModel;
use grams::read_lines;

fn main() {
    // Get program arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough program arguments supplied, you supplied: {:?}", args);
    }
    let file_path = &args[1];

    // Create an instance of the NGramModel
    let mut ngmodel = NGramModel::new(4);

    // File text_sample.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {
        // Use lines from the iterator
        for line in lines.flatten() {
            if !line.is_empty() {
                ngmodel.update_ngram_model(line);
            }
        }
    }

    let most_common_ngram_result = ngmodel.most_common_ngram_without_sentence_tokens();
    
    assert!(most_common_ngram_result.is_ok());
    println!("Most Frequent ngram: {:?}. It occurred {:?} times.", 
        most_common_ngram_result.unwrap().0,
        most_common_ngram_result.unwrap().1
    );

    let most_common_ngram = most_common_ngram_result.unwrap().0.clone();
    let most_common_probability = ngmodel.calculate_ngram_probability(&most_common_ngram);
    println!("The ngram probability was: {:.3}.",
        most_common_probability
    );

    // Generate some text
    let generated_sentence = ngmodel.generate_text(1).first().unwrap().to_string();
    println!("I generated some text for you, it was: {:?}.",
        generated_sentence
    );
}


