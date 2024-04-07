use std::env;

use grams::BigramModel;
use grams::update_bigram_model;
// use grams::calculate_bigram_probability;
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

    // TODO: Filter out '<S>' and '</S>'

    let most_common_bigram = bmodel.bigram_counts
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .ok_or("Couldn't find a bigram");

    println!("Most Frequent Bigram: {:?}. It occurred {:?} times.", 
        most_common_bigram.unwrap().0,
        most_common_bigram.unwrap().1
    );
}


