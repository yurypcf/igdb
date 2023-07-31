/*!
    Module containing analogous Structs to IGDB endpoint responses.
*/
pub mod age_rating_content_description;
pub use age_rating_content_description::AgeRatingContentDescription;

pub mod age_rating;
pub use age_rating::AgeRating;

pub mod alternative_name;
pub use alternative_name::AlternativeName;

pub mod artwork;
pub use artwork::Artwork;

pub mod character_mug_shot;
pub use character_mug_shot::CharacterMugShot;

#[cfg(feature = "collection")]
pub mod collection;
pub use collection::Collection;

pub mod company_logo;
pub use company_logo::CompanyLogo;
pub use company_logo::CompanyLogoResult;

pub mod company_website;
pub use company_website::CompanyWebsite;
pub use company_website::CompanyWebsiteResult;

pub mod company;
pub use company::Company;
pub use company::CompanyResult;

pub mod cover;
pub use cover::Cover;
pub use cover::CoverResult;

pub mod external_game;
pub use external_game::ExternalGame;
pub use external_game::ExternalGameResult;

#[cfg(feature = "game")]
pub mod game;
pub use game::Game;

#[cfg(feature = "character")]
pub mod character;
pub use character::Character;

#[cfg(feature = "genre")]
pub mod genre;
pub use genre::Genre;


#[cfg(feature = "platform")]
pub mod platform;
pub use platform::Platform;

#[cfg(feature = "theme")]
pub mod theme;
pub use theme::Theme;
