use std::ops::{Add, Div, Mul, Sub};

fn parser(command: &str) -> Option<i32> {
    let tokens = command.trim_right_matches('?')
        .split_whitespace().skip(2);

    let mut output = 0;

    // First operation is adding initial input to 0
    let mut op: fn(i32, i32) -> i32 = i32::add;

    for token in tokens {
        match token.parse::<i32>() {
            Ok(num) => output = op(output, num),
            Err(_) => {
                if token == "by" {
                    continue;
                }
                op = match token {
                    "plus" => i32::add,
                    "minus" => i32::sub,
                    "multiplied" => i32::mul,
                    "divided" => i32::div,
                    _ => return None
                }
            }
        }
    }
    Some(output)
}

pub fn answer(command: &str) -> Option<i32> {
    parser(command)
}
