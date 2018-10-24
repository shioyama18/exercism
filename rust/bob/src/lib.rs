pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.to_uppercase() == x && x.contains(char::is_alphabetic) && x.ends_with("?") => "Calm down, I know what I'm doing!",
        x if x.to_uppercase() == x && x.contains(char::is_alphabetic) => "Whoa, chill out!",
        x if x.ends_with("?") => "Sure.",
        x if x.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
