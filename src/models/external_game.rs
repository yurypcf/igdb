use crate::utils::response_handler::timestamp_as_string;
use num_enum::TryFromPrimitive;
use serde::Deserialize;

pub type ExternalGameResult = Vec<ExternalGame>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ExternalGame {
    pub id: u64,
    pub category: Option<i32>,
    pub created_at: Option<i64>,
    pub game: Option<u64>,
    pub name: String,
    pub uid: String,
    pub updated_at: Option<i64>,
    pub url: String,
    pub year: Option<i32>,
    pub media: Option<i32>,
    pub platform: Option<u64>,
    pub countries: Option<Vec<i32>>,
    pub checksum: String,
}

impl ExternalGame {
  pub fn category(&self) -> &'static str {
    Category::as_int(self.category).as_str_name()
  }

  pub fn media(&self) -> &'static str {
    Media::as_int(self.media).as_str_name()
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
pub enum Category {
    Null = 0,
    Steam = 1,
    Gog = 5,
    Youtube = 10,
    Microsoft = 11,
    Apple = 13,
    Twitch = 14,
    Android = 15,
    AmazonAsin = 20,
    AmazonLuna = 22,
    AmazonAdg = 23,
    EpicGameStore = 26,
    Oculus = 28,
    Utomik = 29,
    ItchIo = 30,
    XboxMarketplace = 31,
    Kartridge = 32,
    PlaystationStoreUs = 36,
    FocusEntertainment = 37,
    XboxGamePassUltimateCloud = 54,
    Gamejolt = 55,
}

impl Category {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Category::Null => {
                "CATEGORY_NULL"
            }
            Category::Steam => "STEAM",
            Category::Gog => "GOG",
            Category::Youtube => "YOUTUBE",
            Category::Microsoft => "MICROSOFT",
            Category::Apple => "APPLE",
            Category::Twitch => "TWITCH",
            Category::Android => "ANDROID",
            Category::AmazonAsin => {
                "AMAZON_ASIN"
            }
            Category::AmazonLuna => {
                "AMAZON_LUNA"
            }
            Category::AmazonAdg => "AMAZON_ADG",
            Category::EpicGameStore => {
                "EPIC_GAME_STORE"
            }
            Category::Oculus => "OCULUS",
            Category::Utomik => "UTOMIK",
            Category::ItchIo => "ITCH_IO",
            Category::XboxMarketplace => {
                "XBOX_MARKETPLACE"
            }
            Category::Kartridge => "KARTRIDGE",
            Category::PlaystationStoreUs => {
                "PLAYSTATION_STORE_US"
            }
            Category::FocusEntertainment => {
                "FOCUS_ENTERTAINMENT"
            }
            Category::XboxGamePassUltimateCloud => {
                "XBOX_GAME_PASS_ULTIMATE_CLOUD"
            }
            Category::Gamejolt => "GAMEJOLT",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
      match value {
        Some(num) => Self::try_from(num).unwrap(),
        None => Category::Null,
      }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[repr(i32)]
pub enum Media {
    Null = 0,
    Digital = 1,
    Physical = 2,
}

impl Media {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Media::Null => "MEDIA_NULL",
            Media::Digital => "DIGITAL",
            Media::Physical => "PHYSICAL",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
      match value {
        Some(num) => Self::try_from(num).unwrap(),
        None => Media::Null,
      }
    }
}