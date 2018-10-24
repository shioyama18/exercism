const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "seventeen", "eighteen", "nineteen"
];
const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty",
    "fifty", "sixty", "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 6] = ["thousand", "million",
    "billion", "trillion", "quadrillion", "quintillion"];

pub fn encode(n: u64) -> String {
    match n {
        0 ... 19 => ONES[n as usize].to_string(),
        20... 99 => {
            let (first, second) = (n / 10, n % 10);
            let word = TENS[first as usize].to_string();
            if second == 0 {
                word
            } else {
                format!("{}-{}", word, encode(second))
            }
        },
        100 ... 999 => format_num(n, 100, "hundred"),
        _ => {
            let mut div = 1000u64;
            for i in 0.. {
                if i == 5 || n < div * 1000 {
                    return format_num(n, div, ORDERS[i]);
                }
                div *= 1000;
            }
            panic!("Unreachable")
        }
    }
}

fn format_num(n: u64, div: u64, order: &str) -> String {
    let (first, second) = (n / div, n % div);
    return if second == 0 {
        format!("{} {}", encode(first), order)
    } else {
        format!("{} {} {}", encode(first), order, encode(second))
    };
}
