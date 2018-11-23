
use std::collections::HashMap;

const VALUES: [(u32, Allergen);8] = [(1, Allergen::Eggs),
                                (2, Allergen::Peanuts),
                                (4, Allergen::Shellfish),
                                (8, Allergen::Strawberries),
                                (16, Allergen::Tomatoes),
                                (32, Allergen::Chocolate),
                                (64, Allergen::Pollen),
                                (128, Allergen::Cats)];

pub struct Allergies {
    allergens: Vec<Allergen>
}

#[derive(Debug, PartialEq)]
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

enum AllergenValues {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies_map = HashMap::new();
        for (val, allergen) in VALUES.iter() {
            allergies_map.insert(val, allergen);
        }

        Allergies {
            allergens: vec![]
        }

    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        unimplemented!(
            "Determine if the patient is allergic to the '{:?}' allergen.",
            allergen
        );
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
