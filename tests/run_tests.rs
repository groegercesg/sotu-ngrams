mod tests {
    use grams::BigramModel;
    use grams::update_bigram_model;
    use grams::calculate_bigram_probability;
    use grams::read_lines;

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
