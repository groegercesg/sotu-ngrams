use std::env;

use grams::NGramModel;
use grams::read_lines;

fn main() {
    // Get program arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough program arguments supplied, you supplied: {:?}", args);
    }

    let file_paths = [
        "text_samples/shakespeare_alllines.txt",
        "text_samples/biden_sotu_2024.txt",
        "text_samples/biden_sotu_2022.txt",
    ];

    // Create an instance of the NGramModel
    let mut ngmodel = NGramModel::new(4);

    // Learn the model with these files
    for file_path in file_paths {
        if let Ok(lines) = read_lines(file_path) {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    ngmodel.update_ngram_model(line);
                }
            }
        }
    };

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


