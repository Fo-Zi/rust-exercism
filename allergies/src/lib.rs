pub struct Allergies{
    allergies: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
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

impl Allergen{
    pub fn new(pwr_2_num: u32) -> Option<Allergen> {
        match pwr_2_num {
            1   => Some(Allergen::Eggs),
            2   => Some(Allergen::Peanuts),
            4   => Some(Allergen::Shellfish),
            8   => Some(Allergen::Strawberries),
            16  => Some(Allergen::Tomatoes),
            32  => Some(Allergen::Chocolate),
            64  => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _   => None, 
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {

        let binary_repr = format!("{:b}",score);

        let allergies_vec: Vec<Option<u32>> 
            = binary_repr
                .chars()
                .rev()
                .enumerate()
                .map( |(index, c)| 
                    if c == '1' { 
                        Some(2_u32.pow(index as u32))
                    } 
                    else {
                        None
                    } )
                .collect();

        let allergies = allergies_vec
            .into_iter()
            .flatten()
            .filter(|x| *x < 256)
            .filter_map(Allergen::new)
            .collect::<Vec<Allergen>>();

        Allergies{
            allergies
        }

    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
       self.allergies.iter().any(|x| *x == *allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }

}
