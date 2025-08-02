#[derive(Debug)]
pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
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
        let mut allergies = vec![];

        if dbg!(score & Allergen::Eggs as u32) > 0 {
            allergies.push(Allergen::Eggs);
        }
        if dbg!(score & Allergen::Peanuts as u32) > 0 {
            allergies.push(Allergen::Peanuts);
        }
        if dbg!(score & Allergen::Shellfish as u32) > 0 {
            allergies.push(Allergen::Shellfish);
        }
        if dbg!(score & Allergen::Strawberries as u32) > 0 {
            allergies.push(Allergen::Strawberries);
        }
        if dbg!(score & Allergen::Tomatoes as u32) > 0 {
            allergies.push(Allergen::Tomatoes);
        }
        if dbg!(score & Allergen::Chocolate as u32) > 0 {
            allergies.push(Allergen::Chocolate);
        }
        if dbg!(score & Allergen::Pollen as u32) > 0 {
            allergies.push(Allergen::Pollen);
        }
        if dbg!(score & Allergen::Cats as u32) > 0 {
            allergies.push(Allergen::Cats);
        }

        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
