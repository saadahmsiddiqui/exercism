// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player { health: 100, mana: Some(100), level: self.level });
            }
            return Some(Player { health: 100, mana: None, level: self.level });
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level < 10 {
            if mana_cost > self.health {
                self.health = 0;
                return self.health;
            }
            self.health = self.health - mana_cost;
            return 0;
        } else {
            let current_mana = self.mana.unwrap();

            if current_mana < mana_cost {
                return 0;
            }

            let new_mana = current_mana - mana_cost;

            self.mana = Some(new_mana);
            return mana_cost * 2;
        }
    }
}
