/*!
    Module containing analogous Structs to IGDB endpoint responses.
*/
#[cfg(feature = "game")]
pub mod game;
pub use game::Game;

#[cfg(feature = "character")]
pub mod character;
pub use character::Character;

#[cfg(feature = "genre")]
pub mod genre;
pub use genre::Genre;

#[cfg(feature = "collection")]
pub mod collection;
pub use collection::Collection;

#[cfg(feature = "platform")]
pub mod platform;
pub use platform::Platform;

#[cfg(feature = "theme")]
pub mod theme;
pub use theme::Theme;
