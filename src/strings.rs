pub fn convert_to_pig_latin(str: &String) -> String {
    let str = str.clone();
    let mut translated: Vec<String> = Vec::new();

    for word in str.split_whitespace() {
        let mut new_word = String::new();

        let chars: Vec<char> = word.clone().chars().collect();

        let first_char = chars[0];

        if "aeiouAEIOU".contains(first_char) {
            new_word.push_str(word);
            new_word.push_str("-hay");
        } else {
            let without_first_letter: &String = &chars[1..word.len()].into_iter().collect();
            new_word.push_str(without_first_letter);
            new_word.push_str("-");
            new_word.push(first_char);
            new_word.push_str("ay");
        }

        translated.push(new_word);
    }

    translated.join(" ")
}
