
use std::collections::HashSet;

const VALUES: [(u32, Allergen);8] = [(1, Allergen::Eggs),
                                (2, Allergen::Peanuts),
                                (4, Allergen::Shellfish),
                                (8, Allergen::Strawberries),
                                (16, Allergen::Tomatoes),
                                (32, Allergen::Chocolate),
                                (64, Allergen::Pollen),
                                (128, Allergen::Cats)];

pub struct Allergies {
    allergens: Vec<Allergen>,
    allergens_set: HashSet<Allergen>
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
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
    pub fn new(score: u32) -> Self {
//        let mut allergies_map = HashMap::new();
        let mut allergens = vec![];
        let mut allergens_set = vec![];
        for (val, allergen) in VALUES.iter() {
            if score & val == *val {
                allergens.push(*allergen);
                allergens_set.push(*allergen);
            }
        }

        Allergies {
            allergens: allergens,
            allergens_set: allergens_set.into_iter().collect()
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens_set.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
