use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Character {
    pub id: usize,
    pub akas: Option<Vec<String>>,
    pub checksum: Option<String>,
    pub country_name: Option<String>,
    pub created_at: Option<usize>,
    pub description: Option<String>,
    pub games: Option<Vec<usize>>,
    pub gender: Option<u8>,
    pub mug_shot: Option<usize>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub species: Option<u8>,
    pub updated_at: Option<usize>,
    pub url: Option<String>,
}

impl Character {
    pub fn gender(&self) -> String {
        match self.gender {
            Some(g) => Gender::new(g).translate(),
            _ => String::from("null"),
        }
    }

    pub fn species(&self) -> String {
        match self.species {
            Some(s) => Species::new(s).translate(),
            _ => String::from("null"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
enum Gender {
    Male,
    Female,
    Other,
    Null,
}

impl Gender {
    fn new(int: u8) -> Gender {
        match int {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Other,
            _ => Gender::Null,
        }
    }

    pub fn translate(&self) -> String {
        match self {
            Gender::Male => String::from("Male"),
            Gender::Female => String::from("Female"),
            Gender::Other => String::from("Other"),
            Gender::Null => String::from("null"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
enum Species {
    Human,
    Alien,
    Animal,
    Android,
    Unknown,
    Null,
}

impl Species {
    fn new(int: u8) -> Species {
        match int {
            1 => Species::Human,
            2 => Species::Alien,
            3 => Species::Animal,
            4 => Species::Android,
            5 => Species::Unknown,
            _ => Species::Null,
        }
    }

    pub fn translate(&self) -> String {
        match self {
            Species::Human => String::from("Human"),
            Species::Alien => String::from("Alien"),
            Species::Animal => String::from("Animal"),
            Species::Android => String::from("Android"),
            Species::Unknown => String::from("Unknown"),
            Species::Null => String::from("null"),
        }
    }
}
