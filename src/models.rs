/*!
    Module containing analogous Structs to IGDB endpoint responses.
*/
#[cfg(feature = "age_rating_content_description")]
pub mod age_rating_content_description;
pub use age_rating_content_description::AgeRatingContentDescription;
pub use age_rating_content_description::AgeRatingContentDescriptionResult;

#[cfg(feature = "age_rating")]
pub mod age_rating;
pub use age_rating::AgeRating;
pub use age_rating::AgeRatingResult;

#[cfg(feature = "alternative_name")]
pub mod alternative_name;
pub use alternative_name::AlternativeName;
pub use alternative_name::AlternativeNameResult;

#[cfg(feature = "artwork")]
pub mod artwork;
pub use artwork::Artwork;
pub use artwork::ArtworkResult;

#[cfg(feature = "character_mug_shot")]
pub mod character_mug_shot;
pub use character_mug_shot::CharacterMugShot;
pub use character_mug_shot::CharacterMugShotResult;

#[cfg(feature = "collection")]
pub mod collection;
pub use collection::Collection;
pub use collection::CollectionResult;

#[cfg(feature = "company_logo")]
pub mod company_logo;
pub use company_logo::CompanyLogo;
pub use company_logo::CompanyLogoResult;

#[cfg(feature = "company_website")]
pub mod company_website;
pub use company_website::CompanyWebsite;
pub use company_website::CompanyWebsiteResult;

#[cfg(feature = "company")]
pub mod company;
pub use company::Company;
pub use company::CompanyResult;

#[cfg(feature = "cover")]
pub mod cover;
pub use cover::Cover;
pub use cover::CoverResult;

#[cfg(feature = "external_game")]
pub mod external_game;
pub use external_game::ExternalGame;
pub use external_game::ExternalGameResult;

#[cfg(feature = "franchise")]
pub mod franchise;
pub use franchise::Franchise;
pub use franchise::FranchiseResult;

#[cfg(feature = "game_engine_logo")]
pub mod game_engine_logo;
pub use game_engine_logo::GameEngineLogo;
pub use game_engine_logo::GameEngineLogoResult;

#[cfg(feature = "game")]
pub mod game;
pub use game::Game;
pub use game::GameResult;

#[cfg(feature = "character")]
pub mod character;
pub use character::Character;
pub use character::CharacterResult;

#[cfg(feature = "genre")]
pub mod genre;
pub use genre::Genre;
pub use genre::GenreResult;

#[cfg(feature = "platform")]
pub mod platform;
pub use platform::Platform;
pub use platform::PlatformResult;

#[cfg(feature = "theme")]
pub mod theme;
pub use theme::Theme;
pub use theme::ThemeResult;

#[cfg(feature = "search")]
pub mod search;
pub use search::Search;
pub use search::SearchResult;
