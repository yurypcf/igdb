use serde::Deserialize;
use crate::utils::response_handler::timestamp_as_string;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub age_ratings: Option<Vec<usize>>,
    pub aggregated_rating: Option<f32>,
    pub alternative_names: Option<Vec<usize>>,
    pub artworks: Option<Vec<usize>>,
    pub bundles: Option<Vec<usize>>,
    pub category: Option<u8>,
    pub checksum: Option<String>,
    pub collection: Option<usize>,
    pub cover: Option<usize>,
    pub created_at: Option<i64>,
    pub dlcs: Option<Vec<usize>>,
    pub expanded_games: Option<Vec<usize>>,
    pub expansions: Option<Vec<usize>>,
    pub external_games: Option<Vec<usize>>,
    pub first_release_date: Option<usize>,
    pub follows: Option<usize>,
    pub forks: Option<Vec<usize>>,
    pub franchise: Option<usize>,
    pub franchises: Option<Vec<usize>>,
    pub game_engines: Option<Vec<usize>>,
    pub game_localizations: Option<Vec<usize>>,
    pub game_modes: Option<Vec<usize>>,
    pub genres: Option<Vec<usize>>,
    pub hypes: Option<usize>,
    pub involved_companies: Option<Vec<usize>>,
    pub keywords: Option<Vec<usize>>,
    pub language_supports: Option<Vec<usize>>,
    pub multiplayer_modes: Option<Vec<usize>>,
    pub name: Option<String>,
    pub parent_game: Option<usize>,
    pub platforms: Option<Vec<usize>>,
    pub player_perspectives: Option<Vec<usize>>,
    pub ports: Option<Vec<usize>>,
    pub rating: Option<f32>,
    pub rating_count: Option<usize>,
    pub release_dates: Option<Vec<usize>>,
    pub remakes: Option<Vec<usize>>,
    pub remasters: Option<Vec<usize>>,
    pub screenshots: Option<Vec<usize>>,
    pub similar_games: Option<Vec<usize>>,
    pub slug: Option<String>,
    pub standalone_expansions: Option<Vec<usize>>,
    pub status: Option<u8>,
    pub storyline: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<Vec<usize>>,
    pub themes: Option<Vec<usize>>,
    pub total_rating: Option<f32>,
    pub total_rating_count: Option<usize>,
    pub updated_at: Option<i64>,
    pub url: Option<String>,
    pub version_parent: Option<usize>,
    pub version_title: Option<String>,
    pub videos: Option<Vec<usize>>,
    pub websites: Option<Vec<usize>>,
}

impl Game {
    pub fn category(&self) -> String {
        match self.category {
            Some(g) => Category::new(g).translate(),
            _ => String::from("null"),
        }
    }

    pub fn status(&self) -> String {
        match self.status {
            Some(s) => Status::new(s).translate(),
            _ => String::from("null"),
        }
    }

    pub fn created_at(&self) -> String {
      timestamp_as_string(self.created_at)
    }

    pub fn updated_at(&self) -> String {
      timestamp_as_string(self.updated_at)
    }
}

#[derive(Debug, Clone, Deserialize)]
enum Category {
    MainGame,
    DlcAddon,
    Expansion,
    Bundle,
    StandaloneExpansion,
    Mod,
    Episode,
    Season,
    Remake,
    Remaster,
    ExpandedGame,
    Port,
    Fork,
    Pack,
    Update,
    Null,
}

impl Category {
    fn new(int: u8) -> Category {
        match int {
            0 => Category::MainGame,
            1 => Category::DlcAddon,
            2 => Category::Expansion,
            3 => Category::Bundle,
            4 => Category::StandaloneExpansion,
            5 => Category::Mod,
            6 => Category::Episode,
            7 => Category::Season,
            8 => Category::Remake,
            9 => Category::Remaster,
            10 => Category::ExpandedGame,
            11 => Category::Port,
            12 => Category::Fork,
            13 => Category::Pack,
            14 => Category::Update,
            _ => Category::Null,
        }
    }

    pub fn translate(&self) -> String {
        match self {
            Category::MainGame => String::from("MainGame"),
            Category::DlcAddon => String::from("DlcAddon"),
            Category::Expansion => String::from("Expansion"),
            Category::Bundle => String::from("Bundle"),
            Category::StandaloneExpansion => String::from("StandaloneExpansion"),
            Category::Mod => String::from("Mod"),
            Category::Episode => String::from("Episode"),
            Category::Season => String::from("Season"),
            Category::Remake => String::from("Remake"),
            Category::Remaster => String::from("Remaster"),
            Category::ExpandedGame => String::from("ExpandedGame"),
            Category::Port => String::from("Port"),
            Category::Fork => String::from("Fork"),
            Category::Pack => String::from("Pack"),
            Category::Update => String::from("Update"),
            Category::Null => String::from("null"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
enum Status {
    Released,
    Alpha,
    Beta,
    EarlyAccess,
    Offline,
    Cancelled,
    Rumored,
    Delisted,
    Null,
}

impl Status {
    fn new(int: u8) -> Status {
        match int {
            0 => Status::Released,
            2 => Status::Alpha,
            3 => Status::Beta,
            4 => Status::EarlyAccess,
            5 => Status::Offline,
            6 => Status::Cancelled,
            7 => Status::Rumored,
            8 => Status::Delisted,
            _ => Status::Null,
        }
    }

    pub fn translate(&self) -> String {
        match self {
            Status::Released => String::from("Released"),
            Status::Alpha => String::from("Alpha"),
            Status::Beta => String::from("Beta"),
            Status::EarlyAccess => String::from("EarlyAccess"),
            Status::Offline => String::from("Offline"),
            Status::Cancelled => String::from("Cancelled"),
            Status::Rumored => String::from("Rumored"),
            Status::Delisted => String::from("Delisted"),
            Status::Null => String::from("null"),
        }
    }
}
