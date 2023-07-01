# IGDB API Rust Wrapper
#### [Internet Game Database API](https://www.igdb.com/api) crate. With the igdb crate, you can retrieve information on any of IGDB API endpoints.

If you would like to help the project, pull requests and suggestions are always welcome :)

## Installation
This crate is using [cargo-features](https://doc.rust-lang.org/cargo/reference/features.html) so your project compiles only code related to the endpoint you'll be using.
If you're querying only `games` and `characters` endpoint, `Cargo.toml` should look like this:
```toml
[dependencies]
igdb = { "0.1.0", default-features = false, features = ["game", "character"]}
```
Unless you want the entire codebase from the crate containing all endpoits methods and structs
add this to your `Cargo.toml`:
```toml
[dependencies]
igdb = "0.1.0"
```
## Usage
##### IGDB requires Twitch access credentials to work.
You can read how to retrieve those credentials [here](https://api-docs.igdb.com/#account-creation).
With **Twitch Access Token** and **Twitch Client ID** in hands, we can bring IGDB API wrapper into scope like this:
```rust
// Along with the wrapper, bring the endpoint Structs to scope so your code knows the return type of the Vector
use igdb::{APIWrapper, models::Game, models::Character};
use std::env;

fn main() {
  // Using stored environment variables to unwrap our credentials
  let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
  let client_id = env::var("TWITCH_CLIENT_ID").unwrap();

  // Authenticating with our API wrapper
  let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

  // Using the API wrapper methods to query for Zelda games
  // Here we are expecting a vector of Game struct, so we used the imported Struct.
  let zelda_games: Vec<Game> = api_wrapper.games()
    .search("zelda")
    .limit("2")
    .fields("name")
    .request()
    .unwrap();

  // Using the API wrapper methods to query for Characters named Mario
  // Here we are expecting a vector of Character struct.
  let characters_named_mario: Vec<Character> = api_wrapper.characters()
    .search("mario")
    .fields("name")
    .request()
    .unwrap();
}
```
This example used **environment variables** to store the Twitch retrieved credentials, and then accessing with the rust standard feature `std::env`. In your personal project, you can manage these credentials as you please.


