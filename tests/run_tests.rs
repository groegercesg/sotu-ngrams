mod tests {
    use grams::BigramModel;
    use grams::update_bigram_model;
    use grams::calculate_bigram_probability;
    use grams::read_lines;

    #[test]
    fn basic_line_of_text_number_tokens() {
        let mut bmodel = BigramModel::new();

        let line_of_text = "This year again we are finally together again".to_string();
        update_bigram_model(line_of_text, &mut bmodel);

        // Number of Tokens
        let got = bmodel.token_counts.keys().len();
        let want = 7;
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_contents_tokens() {
        let mut bmodel = BigramModel::new();

        let line_of_text = "This year again we are finally together again".to_string();
        update_bigram_model(line_of_text, &mut bmodel);

        // Contents of Tokens
        let mut got = bmodel.token_counts.into_keys().collect::<Vec<String>>();
        got.sort();
        let mut want = vec!["This".to_string(), "again".into(), "are".into(), "finally".into(), "together".into(), "we".into(), "year".into()];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn basic_line_of_text_values_tokens() {
        let mut bmodel = BigramModel::new();

        let line_of_text = "This year again we are finally together again".to_string();
        update_bigram_model(line_of_text, &mut bmodel);

        // Values of Tokens
        let mut got = bmodel.token_counts.into_values().collect::<Vec<i64>>();
        got.sort();
        let mut want = vec![2, 1, 1, 1, 1, 1, 1];
        want.sort();
        assert_eq!(got, want);
    }


    #[test]
    fn biden_2022_full_test() {
        let mut bmodel = BigramModel::new();

        // File text_sample.txt must exist in the current path
        if let Ok(lines) = read_lines("./biden_sotu_2022.txt") {
            // Use lines from the iterator
            for line in lines.flatten() {
                if !line.is_empty() {
                    update_bigram_model(line, &mut bmodel)
                }
            }
        }

        let test_tuple = ("keep".to_string(), "moving".to_string());
        
        let got = calculate_bigram_probability(&test_tuple, &mut bmodel);
        let want = 0.07692307692307693;

        assert_eq!(got, want);
    }
}
