use std::env;
use std::vec;

use grams::NGramModel;

use select::document::Document;
use select::predicate::Name;
use scraper::{Html, Selector};
use reqwest;

fn main() {
    // Get program arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough program arguments supplied, you supplied: {:?}", args);
    }

    let directory_url = "https://www.presidency.ucsb.edu/documents/presidential-documents-archive-guidebook/annual-messages-congress-the-state-the-union";
    let text = get_text(directory_url);

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

    // Create an instance of the NGramModel
    let mut ngmodel = NGramModel::new(4);

    // Build selector for SOTU content
    let selector = Selector::parse(r#"div[class="field-docs-content"]"#).unwrap();

    for (pos, sotu_link) in sotu_links.iter().enumerate() {
        println!("{:?}/{total_links} -- Downloading: {sotu_link}", pos+1);
        let sotu_text = get_text(&sotu_link);
        match sotu_text {
            Ok(content) => {
                let fragment = Html::parse_fragment(&content);
                let ul = fragment.select(&selector).next().unwrap();
                let text_lines = ul.child_elements().flat_map(|el| el.text()).collect::<Vec<_>>();
                
                // Got the content and loading it into the model
                for line in text_lines {
                    for part_line in line.split(".") {
                        if !part_line.is_empty() {
                            ngmodel.update_ngram_model(part_line.to_string());
                        }
                    }
                }
            }
            Err(e) => panic!("Failed to get text: {e:?}")
        }
    }

    // // Learn the model with these files
    // for file_path in file_paths {
    //     if let Ok(lines) = read_lines(file_path) {
    //         // Use lines from the iterator
    //         for line in lines.flatten() {
    //             if !line.is_empty() {
    //                 ngmodel.update_ngram_model(line);
    //             }
    //         }
    //     }
    // };

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

fn get_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
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
