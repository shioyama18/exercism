use std::ops::BitAnd;

pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        let mut allergies = Vec::new();

        for i in 0..8 {
            if score.bitand(1u32) == 1 {
                let a = match i {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    7 => Allergen::Cats,
                    _ => break
                };
                allergies.push(a);
            }
            score /= 2;
        }

        Allergies(allergies)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }
}
