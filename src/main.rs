use std::env;

use grams::BigramModel;
use grams::read_lines;

fn main() {
    // Get program arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough program arguments supplied, you supplied: {:?}", args);
    }
    let file_path = &args[1];

    // Create an instance of the BigramModel
    let mut bmodel = BigramModel::new(3);

    // File text_sample.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {
        // Use lines from the iterator
        for line in lines.flatten() {
            if !line.is_empty() {
                bmodel.update_ngram_model(line);
            }
        }
    }

    // TODO: Why can't I use both of these back to back?

    // let most_common_bigram = bmodel.most_common_ngram_without_sentence_tokens();
    
    // assert!(most_common_bigram.is_ok());
    // println!("Most Frequent ngram: {:?}. It occurred {:?} times.", 
    //     most_common_bigram.unwrap().0,
    //     most_common_bigram.unwrap().1
    // );

    let most_common_bigram = vec![
        "the".to_string(),
        "United".to_string(),
        "States".to_string()
    ];
    let most_common_probability = bmodel.calculate_ngram_probability(&most_common_bigram);
    println!("The ngram probability was: {:.3}.",
        most_common_probability
    )
}


