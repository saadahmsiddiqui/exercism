#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs, // 1
    Peanuts,// 2
    Shellfish,// 4
    Strawberries,// 8
    Tomatoes,// 16
    Chocolate,// 32
    Pollen,// 64
    Cats,// 128
}

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub struct Allergies {
    allergies: Vec<Allergen>
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let total_score = score;
        if total_score == 255 {
            let all = Vec::from([
                Allergen::Eggs,
                Allergen::Peanuts,
                Allergen::Shellfish,
                Allergen::Strawberries,
                Allergen::Tomatoes,
                Allergen::Chocolate,
                Allergen::Pollen,
                Allergen::Cats,
            ]);
            return Allergies { allergies: all };
        }
        let mut a_vec: Vec<Allergen> = Vec::new();
        for i in 0..8 {

            let bit_at_i = get_bit_at(total_score, i);
            println!("{}", bit_at_i);
            if bit_at_i == true {
                let allerg = match i {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    7 => Allergen::Cats,
                    _ => continue
                };
                a_vec.push(allerg);
            }
        }

        return Allergies { allergies: a_vec }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let self_allergies: &Vec<Allergen> = &self.allergies;
        for i in 0..self_allergies.len() {
            if self_allergies.get(i).unwrap().eq(&allergen) {
                return true;
            }
        }
        return false;
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
