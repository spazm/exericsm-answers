#[derive(Debug)]
pub struct Allergies(pub i32);

#[derive(Copy, Clone, Debug, PartialEq)]
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

static ALL: [Allergen; 8] = [Allergen::Eggs,
                             Allergen::Peanuts,
                             Allergen::Shellfish,
                             Allergen::Strawberries,
                             Allergen::Tomatoes,
                             Allergen::Chocolate,
                             Allergen::Pollen,
                             Allergen::Cats];


impl Allergies {
    /// Tests if self is allergic to allergen
    /// Checks if bit for allergen is set in our score
    pub fn is_allergic_to(&self, allergen : &Allergen) -> bool {
        let &Allergies(score) = self;  // pattern match self to extract allergen bitfield score

        // extract enum value of allergen
        let value = *allergen as i32;

        // is score bit set in allergen?
        // bitwise and of value and score will be non-zero if value-th bit is set
        value & score != 0
    }

    /// Filter list of all Allergens to include only the ones self is alergic to.
    /// ALL is an array of &Allergen, so we must convert to Allergen via `map(|&n| n)`
    pub fn allergies(&self) -> Vec<Allergen> {
        let allergens: Vec<Allergen> = ALL
            .iter()
            .filter( |a| self.is_allergic_to(a))
            .map(|&n| n)
            .collect();
        allergens
    }
}
