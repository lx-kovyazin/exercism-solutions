pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        String::new()
    } else {
        use inflector::Inflector;
        
        phrase
            .to_owned()
            .replace(&['-', '_', ':', ','], " ")
            .split_whitespace()
            .flat_map(|word| {
                if word.is_pascal_case() {
                    word.to_title_case()
                        .split_whitespace()
                        .map(|sub_word| sub_word.to_owned())
                        .collect::<Vec<String>>()
                } else {
                    vec![word.to_owned()]
                }
            })
            .map(|word| word.chars().next().unwrap().to_uppercase().to_string())
            .collect()
    }
}
