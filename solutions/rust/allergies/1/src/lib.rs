pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs         = 0x01,
    Peanuts      = 0x02,
    Shellfish    = 0x04,
    Strawberries = 0x08,
    Tomatoes     = 0x10,
    Chocolate    = 0x20,
    Pollen       = 0x40,
    Cats         = 0x80,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        [
            Eggs        ,
            Peanuts     ,
            Shellfish   ,
            Strawberries,
            Tomatoes    ,
            Chocolate   ,
            Pollen      ,
            Cats        ,
        ]
        .into_iter()
        .filter(|a| self.is_allergic_to(a))
        .collect()
    }
}
