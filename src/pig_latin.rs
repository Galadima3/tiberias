fn is_vowel(c : char) -> bool {
    matches!(c.to_lowercase().next().unwrap(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn to_pig_latin (word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    // Get the first character (properly handling UTF-8)
    let first_char = word.chars().next().unwrap();

    if is_vowel(first_char) {
        format!("{}-hay", word)
    } else {
        let rest = word.chars().skip(1).collect::<String>();
        format!("{}-{}ay", rest, first_char)
    }
}


pub fn convert_sentence(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| to_pig_latin(word).to_lowercase())
        .collect::<Vec<String>>()
        .join(" ")
}