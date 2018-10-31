pub fn abbreviate(phrase: &str) -> String {
    let mut output = String::new();

    for word in phrase.split(|c: char| !c.is_alphabetic()) {
        if word.is_empty() {
            continue;
        } else if word.chars().all(char::is_uppercase) {
            output.push(word.chars().next().unwrap());
        } else if word.chars().all(char::is_lowercase) {
            output.push(word.to_uppercase().chars().next().unwrap());
        } else {
            word.chars().filter(|c| c.is_uppercase()).for_each(|c| output.push(c));
        }
    }
    
    output
}
