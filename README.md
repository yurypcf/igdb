# IGDB API Rust Wrapper

[![Crates.io](https://img.shields.io/crates/v/rusty-igdb.svg)](https://crates.io/crates/rusty_igdb)
[![CI/CD](https://github.com/yurypcf/igdb/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/yurypcf/igdb/actions/workflows/ci-cd.yml)
[![Apache-2 licensed](https://img.shields.io/crates/l/rusty-igdb.svg)](./LICENSE-APACHE)

#### [Internet Game Database API](https://www.igdb.com/api) crate. With the igdb crate, you can retrieve information on any of IGDB API endpoints.

If you would like to help the project, pull requests and suggestions are always welcome :)
## Table Of Contents
- [Installation](#installation)
- [Usage](#usage)
  - [Basic Rust usage](#basic-rust-usage)
  - [JSON response](#json-response)
- [Roadmap](#roadmap-)
- [License](#license)

## Installation
This crate is using [cargo-features](https://doc.rust-lang.org/cargo/reference/features.html) so your project compiles only code related to the endpoint you'll be using.
For example, if you're querying only `games` and `characters` endpoint, `Cargo.toml` should look like this:

```toml
[dependencies]
rusty_igdb = { "2.0.0", default-features = false, features = ["game", "character"]}
```
Unless you want the entire codebase from the crate containing all endpoits methods and structs
add this to your `Cargo.toml`:
```toml
[dependencies]
rusty_igdb = "2.0.0"
```

Also, this package uses [rust nightly build](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html). Be sure to have installed.

## Usage

#### Basic Rust usage

##### IGDB requires Twitch access credentials to work.
You can read how to retrieve those credentials [here](https://api-docs.igdb.com/#account-creation).
With **Twitch Access Token** and **Twitch Client ID** in hands, we can bring IGDB API wrapper into scope like this:
```rust
// Along with the wrapper, bring the endpoint Structs to scope so your code knows the return type of the Vector
use rusty_igdb::{ APIWrapper, models::{ Game, GameResult, Character, CharacterResult  } };
use std::env;

fn main() {
  // Using stored environment variables to unwrap our credentials
  let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
  let client_id = env::var("TWITCH_CLIENT_ID").unwrap();

  // Authenticating with our API wrapper
  let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

  // Using the API wrapper methods to query for Zelda games
  // Here we are expecting a vector of Game struct, so we used the imported Struct.
  let zelda_games: GameResult = api_wrapper
    .build("games") // endpoint
    .search("zelda")
    .limit("2")
    .fields("name")
    .request()
    .unwrap();

  // Using the API wrapper methods to query for Characters named Mario
  // Here we are expecting a vector of Character struct.
  let characters_named_mario: CharacterResult = api_wrapper
    .build("characters") // endpoint
    .search("mario")
    .fields("name")
    .request()
    .unwrap();
}
```
This example used **environment variables** to store the Twitch retrieved credentials, and then accessing with the rust standard feature `std::env`. In your personal project, you can manage these credentials as you please.

You can avoid typing `fields()` query method if you want a full fields response
This would return all fields related to characters that contains Solid Snake

```rust
  // no .fields() query method returns every field from endpoint
  let solid_snake_chars_vec: CharacterResult = api_wrapper
    .build("characters")
    .search("Solid Snake")
    .request()
    .unwrap();
```
#### JSON response

The crate offers a JSON public method, so you can customize the response content into your project.
`Value` represents the `serde_json::Value` struct in the below example.
[See the serde_json crate for more information about.](https://docs.rs/serde_json/latest/serde_json/value/index.html)

All IGDB endpoints should be available using the `request_json()` method.

```rust
    let test_characters: Vec<Value> = api_wrapper
      .build("search")
      .fields("name, gender, country_name")
      .where_like("gender != null")
      .limit("2")
      .request_json()
      .unwrap();
    
    /*
    response should look like this:
      [{
        "gender": 0,
        "id": 4445,
        "name": "Beast"
      },
      {
        "gender": 0,
        "id": 8988,
        "name": "Mr. Wong"
      }],
    */
```

The result should look and accessed like this:

```json
      [{
        "gender": 0,
        "id": 4445,
        "name": "Beast"
      },
      {
        "gender": 0,
        "id": 8988,
        "name": "Mr. Wong"
      }]
```
```rust
  let first_character = &test_characters[0]["gender"];
```

## Roadmap

- Remaining IGDB Endpoints (see [milestone](https://github.com/yurypcf/igdb/milestone/1))
- Add code coverage

## License
Cargo is primarily distributed under the terms of Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE)

