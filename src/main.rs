use std::env;
use std::vec;

use grams::NGramModel;
use grams::read_lines;

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

    let file_paths = [
        "text_samples/shakespeare_alllines.txt",
        "text_samples/biden_sotu_2024.txt",
        "text_samples/biden_sotu_2022.txt",
    ];

    let directory_url = "https://www.presidency.ucsb.edu/documents/presidential-documents-archive-guidebook/annual-messages-congress-the-state-the-union";
    let text = get_text(directory_url);

    println!("Parsing for links");

    let mut sotu_links: Vec<String> = vec![];

    match text {
        Ok(content) => {
            Document::from(content.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .filter(|a| a.contains("https://www.presidency.ucsb.edu/documents/"))
                .for_each(|x| sotu_links.push(x.to_string()));
        }
        Err(e) => panic!("Failed to get text: {e:?}")
    }


    sotu_links.sort_unstable();
    sotu_links.dedup();
    let total_links = sotu_links.len();
    println!("{:?}", total_links);
    // for link in sotu_links {
    //     println!("{link}")
    // }

    // Create an instance of the NGramModel
    let mut ngmodel = NGramModel::new(4);

    let selector = Selector::parse(r#"div[class="field-docs-content"]"#).unwrap();


    for (pos, sotu_link) in sotu_links.iter().enumerate() {
        let sotu_text = get_text(&sotu_link);
        println!("{:?}/{total_links} -- Doing: {sotu_link}", pos+1);
        match sotu_text {
            Ok(content) => {
                let fragment = Html::parse_fragment(&content);
                let ul = fragment.select(&selector).next().unwrap();
                let text_lines = ul.child_elements().flat_map(|el| el.text()).collect::<Vec<_>>();
                
                // Got the content and loading it into the model

                // TODO - Split on fullstop

                for line in text_lines {
                    if !line.is_empty() {
                        ngmodel.update_ngram_model(line.to_string());
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


