pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut sum = 0;
        let mut len = 0;

        for (i, c) in self.0.chars().filter(|c| !c.is_whitespace()).rev().enumerate() {
            len += 1;
            match (i % 2, c.to_digit(10)) {
                (1, Some(x)) if x > 4 => sum += x * 2 - 9,
                (1, Some(x)) => sum += x * 2,
                (0, Some(x)) => sum += x,
                (_, _) => return false,
            }
        }

        (len > 1) && (sum % 10 == 0)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        Luhn(input.into())
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn(input)
    }
}

macro_rules! define_from_unsigned_int {
    ($($x:ident),*) => {
        $(
            impl From<$x> for Luhn {
                fn from(input: $x) -> Self {
                    Luhn(input.to_string())
                }
            }
        )*
    };
    ($($x:ident,)*) => {
        define_from_unsigned_int!($($x),*);
    };
}

define_from_unsigned_int!(u8, u16, u32, u64, usize,);


