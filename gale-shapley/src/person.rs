enum Gender {
    Male,
    Female,
}

pub enum Nationality {
    Chinese,
    Russian,
    British,
    NoMatter,
}

struct Characteristic {
    growth: f32,
    weight: u8,
    nationality: Nationality,
}

impl Default for Characteristic {
    fn default() -> Self {
        Self {
            growth: Default::default(),
            weight: Default::default(),
            nationality: Nationality::Chinese,
        }
    }
}

struct Rreference {
    growth: (f32, f32),
    weight: (u8, u8),
    nationality: Nationality,
}

impl Default for Rreference {
    fn default() -> Self {
        Self {
            growth: Default::default(),
            weight: Default::default(),
            nationality: Nationality::Chinese,
        }
    }
}

#[allow(dead_code)]
pub struct Person {
    name: &'static str,
    gender: Gender,
    characteristics: Characteristic,
    preferences: Rreference,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Alex",
            gender: Gender::Male,
            characteristics: Default::default(),
            preferences: Default::default(),
        }
    }
}

impl Person {
    fn new(name: &'static str, gender: Gender) -> Person {
        Self {
            name,
            gender,
            ..Default::default()
        }
    }

    pub fn male(name: &'static str) -> Person {
        Self::new(name, Gender::Male)
    }

    pub fn female(name: &'static str) -> Person {
        Self::new(name, Gender::Female)
    }

    pub fn characteristics(mut self, growth: f32, weight: u8, nationality: Nationality) -> Person {
        self.characteristics.growth = growth;
        self.characteristics.weight = weight;
        self.characteristics.nationality = nationality;
        self
    }

    pub fn preferences(
        mut self,
        growth: (f32, f32),
        weight: (u8, u8),
        nationality: Nationality,
    ) -> Person {
        self.preferences.growth = growth;
        self.preferences.weight = weight;
        self.preferences.nationality = nationality;
        self
    }
}
