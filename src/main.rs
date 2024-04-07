use std::env;

use grams::BigramModel;
use grams::update_bigram_model;
use grams::calculate_bigram_probability;
use grams::read_lines;

fn main() {
    // Get program arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough program arguments supplied, you supplied: {:?}", args);
    }
    let file_path = &args[1];

    // Create an instance of the BigramModel
    let mut bmodel = BigramModel::new();

    // File text_sample.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {
        // Use lines from the iterator
        for line in lines.flatten() {
            if !line.is_empty() {
                update_bigram_model(line, &mut bmodel)
            }
        }
    }

    let test_tuple = ("keep".to_string(), "moving".to_string());
    let prob_value = calculate_bigram_probability(&test_tuple, &mut bmodel);

    println!("{:?}: {:.3}", 
        test_tuple,
        prob_value
    );
}


