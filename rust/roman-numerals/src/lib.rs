use std::fmt::{Display, Formatter, Result};

static SEQUENCE: [(u32, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut roman = String::new();
        for &(n_i, r_i) in SEQUENCE.into_iter() {
            while num >= n_i {
                roman.push_str(r_i);
                num -= n_i;
            }
        }
        Roman(roman)
    }
}
