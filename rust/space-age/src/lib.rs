// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / 31557600.0)
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::period()
    }
}

macro_rules! create_planet {
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;
        impl Planet for $name {
            fn period() -> f64 {
                $orbital_period
            }
        }
    }
}

create_planet!(Earth, 1.0);
create_planet!(Mercury, 0.2408467);
create_planet!(Venus, 0.61519726);
create_planet!(Mars, 1.8808158);
create_planet!(Jupiter, 11.862615);
create_planet!(Saturn, 29.447498);
create_planet!(Uranus, 84.016846);
create_planet!(Neptune, 164.79132);
