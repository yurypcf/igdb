use crate::utils::response_handler::timestamp_as_string;
use num_enum::TryFromPrimitive;
use serde::Deserialize;

pub type CharacterResult = Vec<Character>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Character {
    pub id: u64,
    pub akas: Option<Vec<String>>,
    pub country_name: Option<String>,
    pub created_at: Option<i64>,
    pub description: Option<String>,
    pub games: Option<Vec<u64>>,
    pub gender: Option<i32>,
    pub mug_shot: Option<i32>,
    pub name: String,
    pub slug: String,
    pub species: Option<i32>,
    pub updated_at: Option<i64>,
    pub url: String,
    pub checksum: String,
}

impl Character {
    pub fn gender(&self) -> &'static str {
        Gender::as_int(self.gender).as_str_name()
    }

    pub fn species(&self) -> &'static str {
        Species::as_int(self.species).as_str_name()
    }

    pub fn created_at(&self) -> String {
        timestamp_as_string(self.created_at)
    }

    pub fn updated_at(&self) -> String {
        timestamp_as_string(self.updated_at)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[repr(i32)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Other = 2,
}

impl Gender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Gender::Male => "MALE",
            Gender::Female => "FEMALE",
            Gender::Other => "OTHER",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
        match value {
            Some(num) => Self::try_from(num).unwrap(),
            None => Gender::Other,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[repr(i32)]
pub enum Species {
    Null = 0,
    Human = 1,
    Alien = 2,
    Animal = 3,
    Android = 4,
    Unknown = 5,
}
impl Species {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Species::Null => "CHARACTER_SPECIES_NULL",
            Species::Human => "HUMAN",
            Species::Alien => "ALIEN",
            Species::Animal => "ANIMAL",
            Species::Android => "ANDROID",
            Species::Unknown => "UNKNOWN",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
        match value {
            Some(num) => Self::try_from(num).unwrap(),
            None => Species::Null,
        }
    }
}
