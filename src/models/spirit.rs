use std::{borrow::Borrow, fmt};

use super::species;

pub struct Spirit {
    pub name: String,

    pub hp_current: i32,
    pub hp_max: i32,

    pub attack_current: i32,
    pub defense_current: i32,

    pub speed_current: i32,

    pub species: species::SpeciesType,
}

impl fmt::Display for Spirit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Spirit {} (HP: {}/{})",
            self.name, self.hp_current, self.hp_max
        )
    }
}

impl Spirit {
    /**
     * 攻击方法
     */
    pub fn attack(&mut self, other_spirit: &mut Spirit) {
        let damage_multiplier = species::get_attack_rate(self.species.borrow(), other_spirit.species.borrow());
        let damage: f32 = damage_multiplier * self.attack_current as f32;
        other_spirit.hp_current -= damage.floor() as i32;
    }

    pub fn get_default_spirit() -> Spirit {
        Spirit {
            name: String::from("默认小精灵"),
            hp_current: 100,
            hp_max: 100,
            attack_current: 5,
            speed_current: 1,
            defense_current: 5,
            species: species::SpeciesType::Normal,
        }
    }

    pub fn get_primary_hydro_spirit() -> Spirit {
        Spirit {
            name: String::from("艾特慕斯"),
            hp_current: 100,
            hp_max: 100,
            attack_current: 10,
            speed_current: 1,
            defense_current: 5,
            species: species::SpeciesType::Hydro,
        }
    }
    pub fn get_primary_flare_spirit() -> Spirit {
        Spirit {
            name: String::from("放克申"),
            hp_current: 80,
            hp_max: 80,
            attack_current: 20,
            speed_current: 2,
            defense_current: 5,
            species: species::SpeciesType::Flare,
        }
    }

    pub fn get_primary_floral_spirit() -> Spirit {
        Spirit {
            name: String::from("奥博杰克特"),
            hp_current: 150,
            hp_max: 150,
            attack_current: 10,
            speed_current: 1,
            defense_current: 10,
            species: species::SpeciesType::Floral,
        }
    }
}
