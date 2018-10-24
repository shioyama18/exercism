pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = Vec::new();

    for c in string.chars() {
        match c {
            '{' | '[' | '(' => s.push(c),
            '}' | ']' | ')' => 
                match s.pop() {
                    Some('{') => if c != '}' { return false; },
                    Some('[') => if c != ']' { return false; },
                    Some('(') => if c != ')' { return false; },
                    _ => continue,
                }
            _ => continue,
        }
    }

    s.is_empty()
}
