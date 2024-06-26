mod tests {
    use grams::NGramModel;
    use grams::read_lines;

    #[test]
    fn generate_text_trigram_biden_2024() {
        let mut bmodel = NGramModel::new(3);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2024.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let got = bmodel.generate_text("Greedy".to_string(), 1).first().unwrap().to_string();
        
        let want = "I see a future where we all come from somewhere but we are".to_string();
        assert_eq!(got, want);
    }

    #[test]
    fn generate_text_quadgram_biden_2022() {
        let mut bmodel = NGramModel::new(4);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let got = bmodel.generate_text("Greedy".to_string(), 1).first().unwrap().to_string();
        
        let want = "And we will as one people".to_string();
        assert_eq!(got, want);
    }

    #[test]
    fn generate_text_unigram_biden_2022() {
        let mut bmodel = NGramModel::new(1);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let got = bmodel.generate_text("Greedy".to_string(), 1).first().unwrap().to_string();
        
        // This will be the most probable word 25 times, the limit before max sentence
        let want = "the the the the the the the the the the the the the the the the the the the the the the the the the".to_string();
        assert_eq!(got, want);
    }

    #[test]
    fn probability_of_sentence_bigram_valid() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together are again we".to_string();
        bmodel.update_ngram_model(line_of_text);

        let test_sentence = "This year again we are again".to_string();
        let got = bmodel.probability_of_sentence(test_sentence);
        let want = 0.022727272727272728;
        assert_eq!(got, want);
    }

    #[test]
    fn probability_of_sentence_bigram() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again we".to_string();
        bmodel.update_ngram_model(line_of_text);

        let test_sentence = "This year again we are not happy".to_string();
        let got = bmodel.probability_of_sentence(test_sentence);
        let want = 0.0;
        assert_eq!(got, want);
    }

    #[test]
    fn probability_of_sentence_trigram() {
        let mut bmodel = NGramModel::new(3);

        let line_of_text = "This year again we are finally together again we".to_string();
        bmodel.update_ngram_model(line_of_text);

        let test_sentence = "This is a sentence we want to test".to_string();
        let got = bmodel.probability_of_sentence(test_sentence);
        let want = 0.0;
        assert_eq!(got, want);
    }

    #[test]
    fn prob_of_partial_gram_bigram() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again we".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Number of Tokens
        let got = bmodel.ngram_counts.len();
        let want = 9;
        assert_eq!(got, want);

        let partial_ngram = vec!["again".to_string()];
        let got = bmodel.probability_for_partial_ngram(&partial_ngram);
        let want = 2 as f64 / 10 as f64;
        assert_eq!(got, want);
    }

    #[test]
    fn prob_of_partial_gram_trigram() {
        let mut bmodel = NGramModel::new(3);

        let line_of_text = "mango test test mango cake test mango cake monkey cake test mango cake".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Number of Tokens
        let got = bmodel.ngram_counts.len();
        let want = 12;
        assert_eq!(got, want);

        let partial_ngram = vec!["mango".to_string(), "test".to_string()];
        let got = bmodel.probability_for_partial_ngram(&partial_ngram);
        let want = 1 as f64 / 4 as f64;
        assert_eq!(got, want);

        let partial_ngram = vec!["cake".to_string(), "test".to_string()];
        let got = bmodel.probability_for_partial_ngram(&partial_ngram);
        let want = 2 as f64 / 4 as f64;
        assert_eq!(got, want);

        let partial_ngram = vec!["mango".to_string(), "cake".to_string()];
        let got = bmodel.probability_for_partial_ngram(&partial_ngram);
        let want = 3 as f64 / 4 as f64;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_number_pen_grams() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Number of Tokens
        let got = bmodel.penultimate_gram_counts.keys().len();
        let want = 8;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_keys_pen_grams() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Keys of Tokens
        let mut got = bmodel.penultimate_gram_counts.into_keys().collect::<Vec<Vec<String>>>();
        got.sort();
        let mut want = vec![
            vec!["<S>".to_string()],
            vec!["This".to_string()],
            vec!["again".into()],
            vec!["are".into()],
            vec!["finally".into()],
            vec!["together".into()],
            vec!["we".into()],
            vec!["year".into()]
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_values_pen_grams() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Values of Tokens
        let mut got = bmodel.penultimate_gram_counts.into_values().collect::<Vec<i64>>();
        got.sort();
        let mut want = vec![2, 1, 1, 1, 1, 1, 1, 1];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_keys_pen_grams_given_last() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Keys of Tokens
        let mut got = bmodel.last_given_penultimate_counts.into_keys().collect::<Vec<Vec<String>>>();
        got.sort();
        let mut want = vec![
            vec!["<S>".to_string()],
            vec!["This".to_string()],
            vec!["again".into()],
            vec!["are".into()],
            vec!["finally".into()],
            vec!["together".into()],
            vec!["we".into()],
            vec!["year".into()]
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_number_bigrams_given_last() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Number of Bigrams
        let got = bmodel.last_given_penultimate_counts.keys().len();
        let want = 8;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_keys_bigrams() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Keys of Bigrams
        let mut got = bmodel.ngram_counts.into_keys().collect::<Vec<Vec<String>>>();
        got.sort();
        let mut want = vec![
            vec!["<S>".to_string(), "This".to_string()],
            vec!["This".to_string(), "year".to_string()],
            vec!["year".to_string(), "again".to_string()],
            vec!["again".to_string(), "we".to_string()],
            vec!["we".to_string(), "are".to_string()],
            vec!["are".to_string(), "finally".to_string()],
            vec!["finally".to_string(), "together".to_string()],
            vec!["together".to_string(), "again".to_string()],
            vec!["again".to_string(), "</S>".to_string()],
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn penta_gram_edge_case() {
        let mut bmodel = NGramModel::new(5);

        let line_of_text = "This".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Keys of Ngrams
        let mut got = bmodel.ngram_counts.into_keys().collect::<Vec<Vec<String>>>();
        got.sort();
        let mut want = vec![
            vec!["<S>".to_string(), "<S>".to_string(), "<S>".to_string(), "<S>".to_string(), "This".to_string()],
            vec!["<S>".to_string(), "<S>".to_string(), "<S>".to_string(), "This".to_string(), "</S>".to_string()],
            vec!["<S>".to_string(), "<S>".to_string(), "This".to_string(), "</S>".to_string(), "</S>".to_string()],
            vec!["<S>".to_string(), "This".to_string(), "</S>".to_string(), "</S>".to_string(), "</S>".to_string()],
            vec!["This".to_string(), "</S>".to_string(), "</S>".to_string(), "</S>".to_string(), "</S>".to_string()],
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_keys_trigrams() {
        let mut bmodel = NGramModel::new(3);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Keys of Bigrams
        let mut got = bmodel.ngram_counts.into_keys().collect::<Vec<Vec<String>>>();
        got.sort();
        let mut want = vec![
            vec!["<S>".to_string(), "<S>".to_string(), "This".to_string()],
            vec!["<S>".to_string(), "This".to_string(), "year".to_string()],
            vec!["This".to_string(), "year".to_string(), "again".to_string()],
            vec!["year".to_string(), "again".to_string(), "we".to_string()],
            vec!["again".to_string(), "we".to_string(), "are".to_string()],
            vec!["we".to_string(), "are".to_string(), "finally".to_string()],
            vec!["are".to_string(), "finally".to_string(), "together".to_string()],
            vec!["finally".to_string(), "together".to_string(), "again".to_string()],
            vec!["together".to_string(), "again".to_string(), "</S>".to_string()],
            vec!["again".to_string(), "</S>".to_string(), "</S>".to_string()],
        ];
        want.sort();
        assert_eq!(got, want);
    }


    #[test]
    fn minimal_line_of_text_filter_punctuation_bigrams() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This! year agai-n we are final/ly together. again.".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Keys of Bigrams
        let mut got = bmodel.ngram_counts.into_keys().collect::<Vec<Vec<String>>>();
        got.sort();
        let mut want = vec![
            vec!["<S>".to_string(), "This".to_string()],
            vec!["This".to_string(), "year".to_string()],
            vec!["year".to_string(), "again".to_string()],
            vec!["again".to_string(), "we".to_string()],
            vec!["we".to_string(), "are".to_string()],
            vec!["are".to_string(), "finally".to_string()],
            vec!["finally".to_string(), "together".to_string()],
            vec!["together".to_string(), "again".to_string()],
            vec!["again".to_string(), "</S>".to_string()],
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_values_bigrams() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Values of Bigrams
        let mut got = bmodel.ngram_counts.into_values().collect::<Vec<i64>>();
        got.sort();
        let mut want = vec![1,1,1,1,1,1,1,1,1];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_basic_probability() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "This year again we are finally together again".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Probability P("we" | "again") = 0.5
        let sample_bigram = vec!["again".to_string(), "we".to_string()];
        let got = NGramModel::calculate_ngram_probability(
            &bmodel.penultimate_gram_counts,
            &bmodel.last_given_penultimate_counts,
            &sample_bigram
        );
        let want = 1 as f64 / 2 as f64;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_no_probability() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "mango test test mango monkey mango cake test mango".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Probability P("monkey" | "dogs") = 0
        let sample_bigram = vec!["dogs".to_string(), "monkey".to_string()];
        let got = NGramModel::calculate_ngram_probability(
            &bmodel.penultimate_gram_counts,
            &bmodel.last_given_penultimate_counts,
            &sample_bigram
        );
        let want = 0 as f64;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_advanced_probability() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "mango test test mango cake mango monkey test mango".to_string();
        bmodel.update_ngram_model(line_of_text);

        // Probability P("cake" | "mango") = 1 / 4
        let sample_bigram = vec!["mango".to_string(), "cake".to_string()];
        let got = NGramModel::calculate_ngram_probability(
            &bmodel.penultimate_gram_counts,
            &bmodel.last_given_penultimate_counts,
            &sample_bigram
        );
        let want = 1 as f64 / 4 as f64;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_most_common() {
        let mut bmodel = NGramModel::new(2);

        let line_of_text = "mango test test mango cake mango monkey test mango".to_string();
        bmodel.update_ngram_model(line_of_text);

        let got = bmodel.most_common_ngram();
        assert!(got.is_ok());

        let want_bigram = vec!["test".to_string(), "mango".to_string()];
        assert_eq!(*got.unwrap().0, want_bigram);
        let want_count = 2;
        assert_eq!(*got.unwrap().1, want_count);
    }

    #[test]
    fn biden_2022_most_common() {
        let mut bmodel = NGramModel::new(2);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let got = bmodel.most_common_ngram();
        assert!(got.is_ok());

        let want_bigram = vec!["<S>".to_string(), "And".to_string()];
        assert_eq!(*got.unwrap().0, want_bigram);
        let want_count = 43;
        assert_eq!(*got.unwrap().1, want_count);
    }

    #[test]
    fn biden_2022_most_common_without_sentence_tokens() {
        let mut bmodel = NGramModel::new(2);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let got = bmodel.most_common_ngram_without_sentence_tokens();
        assert!(got.is_ok());

        let want_bigram = vec!["of".to_string(), "the".to_string()];
        assert_eq!(*got.unwrap().0, want_bigram);
        let want_count = 27;
        assert_eq!(*got.unwrap().1, want_count);
    }

    #[test]
    fn biden_2022_full_test() {
        let mut bmodel = NGramModel::new(2);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let test_tuple = vec!["keep".to_string(), "moving".to_string()];
        
        let got = NGramModel::calculate_ngram_probability(
            &bmodel.penultimate_gram_counts,
            &bmodel.last_given_penultimate_counts,
            &test_tuple
        );
        let want = 0.15384615384615385;

        assert_eq!(got, want);
    }

    #[test]
    fn biden_2022_3_degree_full_test() {
        let mut bmodel = NGramModel::new(3);

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("text_samples/biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    bmodel.update_ngram_model(line);
                }
            }
        }

        let test_tuple = vec!["the".to_string(), "United".to_string(), "States".to_string()];
        
        let got = NGramModel::calculate_ngram_probability(
            &bmodel.penultimate_gram_counts,
            &bmodel.last_given_penultimate_counts,
            &test_tuple
        );
        let want = 0.8571428571428571;

        assert_eq!(got, want);
    }
}
