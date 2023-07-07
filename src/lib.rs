pub mod models;
mod utils;

use reqwest::{ header:: { HeaderMap, HeaderValue }, blocking::{ Client, Response } };
use serde::de::DeserializeOwned;
use utils::{ EndpointUtils, response_handler:: { Result, APIError } };

const BASE_URL: &str = "https://api.igdb.com";
const VERSION:  &str = "v4";

pub struct APIWrapper {
    http_client: Client,
}

impl APIWrapper {
    pub fn new(
        access_token: &str,
        client_id: &str
    ) -> Result<APIWrapper> {
        let mut headers: HeaderMap = HeaderMap::new();
        
        headers.insert("Authorization", format!("Bearer {}", access_token).parse().unwrap());
        headers.insert("Client-ID", HeaderValue::from_str(client_id).unwrap());
    
        let http_client: Client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let wrapper = APIWrapper { http_client };

        Ok(wrapper)
    }

    fn post<D>(
      &self,
      body: String,
      request_endpoint: &str
    ) -> Result<Response>
    where
      D: DeserializeOwned,
    {
      let url = format!("{}/{}/{}", BASE_URL, VERSION, request_endpoint);
      match self.http_client.post(url).body(body).send() {
        Ok(res) => Ok(res),
        Err(err) => Err(APIError::from(err)),
      }
    }

    #[cfg(feature = "game")]
    pub fn games<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "games"}
    }

    #[cfg(feature = "character")]
    pub fn characters<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "characters"}
    }

    #[cfg(feature = "genre")]
    pub fn genres<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "genres"}
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use models::*;
  use std::env;

  // Testing FIELDS apicalypse query
  #[test]
  fn fields_test() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let game: Vec<Game> = api_wrapper.games()
      .fields("*")
      .limit("1")
      .request()
      .unwrap();

    let expected_result: Vec<Game> = vec![
      Game {
        id: 176032,
        age_ratings: Some(vec![140062, 140063, 140064, 140065, 140066]),
        aggregated_rating: None,
        alternative_names: None,
        artworks: Some(vec![108582]),
        bundles: None,
        category: Some(0),
        checksum: Some(String::from("56ca73b8-3a8b-80cc-92f0-cc7b1ea66766")),
        collection: None,
        cover: Some(185893),
        created_at: Some(1634075115),
        dlcs: None,
        expanded_games: None,
        expansions: None,
        external_games: Some(vec![2130968, 2706463]),
        first_release_date: Some(1568505600),
        follows: None,
        forks: None,
        franchise: None,
        franchises: None,
        game_engines: Some(vec![448]),
        game_localizations: None,
        game_modes: Some(vec![1]),
        genres: Some(vec![4, 12, 16, 31]),
        hypes: None,
        involved_companies: Some(vec![153011]),
        keywords: None,
        language_supports: Some(vec![699450]),
        multiplayer_modes: Some(vec![16615]),
        name: Some(String::from("Nick Quest")),
        parent_game: None,
        platforms: Some(vec![6]),
        player_perspectives: None,
        ports: None,
        rating: None,
        rating_count: None,
        release_dates: Some(vec![319506]),
        remakes: None,
        remasters: None,
        screenshots: Some(vec![662044, 662045, 662046, 662047, 662048]),
        similar_games: Some(vec![13196, 37382, 55199, 81249, 96217, 101608, 103303, 106987, 112191, 115653]),
        slug: Some(String::from("nick-quest")),
        standalone_expansions: None,
        status: None,
        storyline: None,
        summary: Some(String::from("Nick Quest is a quirky, self-aware little RPG with a lot of character and heart. Packed with detailed maps, dangerous bosses, a great breadth of side quests, amazing dialogue, multiple endings, and a lot of laughs... Nick Quest is a game that will bring you to know a guy and his friends.")),
        tags: Some(vec![17, 268435460, 268435468, 268435472, 268435487]),
        themes: Some(vec![17]),
        total_rating: None,
        total_rating_count: None,
        updated_at: Some(1686307285),
        url: Some(String::from("https://www.igdb.com/games/nick-quest")),
        version_parent: None,
        version_title: None,  
        videos: None,
        websites: Some(vec![235219, 235220, 472706, 553716, 553717]),
      }
    ];

    assert_eq!(&expected_result, &game)
  }

  // Testing EXCLUDE apicalypse query
  #[test]
  fn exclude_test() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let genres_without_slug_field: Vec<Genre> = api_wrapper.genres()
      .fields("name, slug")
      .exclude("slug")
      .limit("3")
      .request()
      .unwrap();

    let expected_result: Vec<Genre> = vec![
      Genre { id: 4, checksum: None, created_at: None, name: Some(String::from("Fighting")), slug: None, updated_at: None, url: None },
      Genre { id: 5, checksum: None, created_at: None, name: Some(String::from("Shooter")), slug: None, updated_at: None, url: None },
      Genre { id: 7, checksum: None, created_at: None, name: Some(String::from("Music")), slug: None, updated_at: None, url: None }
    ];

    assert_eq!(&expected_result, &genres_without_slug_field)
  }

  // Testing WHERE apicalypse query
  #[test]
  fn where_test() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let test_characters: Vec<Character> = api_wrapper.characters()
      .fields("*")
      .where_like("gender != null")
      .where_like("species != null")
      .request()
      .unwrap();

    let expected_character_result = Character {
      id: 4445,
      akas: None,
      country_name: None,
      description: None,
      created_at: Some(1431216000),
      games: Some(vec![380, 1219, 1221, 2993]),
      gender: Some(0),
      mug_shot: Some(3620),
      name: Some(String::from("Beast")),
      slug: Some(String::from("beast")),
      species: Some(5),
      updated_at: Some(1472601600),
      url: Some(String::from("https://www.igdb.com/characters/beast")),
      checksum: Some(String::from("eb661aaf-a1e1-4acf-b48a-14b8aaa26a52"))
    };

    assert_eq!(&test_characters[0], &expected_character_result);
    assert_eq!(&test_characters[0].gender(), "Male");
    assert_eq!(&test_characters[0].species(), "Unknown");
  }

  // Testing SEARCH apicalypse query
  #[test]
  fn search_test() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let ryu_character: Vec<Character> = api_wrapper.characters()
      .fields("id, name")
      .search("ryu")
      .limit("1")
      .request()
      .unwrap();

    let expected_result: Vec<Character> = vec![
      Character {
        id: 4975,
        akas: None,
        checksum: None,
        country_name: None,
        created_at: None,
        description: None,
        games: None,
        gender: None,
        mug_shot: None,
        name: Some(String::from("Ryu Hyabusa")),
        slug: None,
        species: None,
        updated_at: None,
        url: None
      },
    ];

    assert_eq!(&expected_result, &ryu_character)
  }

  // Testing LIMIT, OFFSET, SORT ASC, SORT DESC apicalypse queries
  #[test]
  fn sorting_test() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let games_limited_by_5_asc: Vec<Game> = api_wrapper.games()
      .fields("name")
      .limit("5")
      .sort_asc("id")
      .request()
      .unwrap();

    assert_eq!(5, games_limited_by_5_asc.len());
    assert_eq!(1, games_limited_by_5_asc[0].id);

    let games_desc_order: Vec<Game> = api_wrapper.games()
      .fields("name")
      .limit("1")
      .sort_desc("id")
      .request()
      .unwrap();

    let last_id = games_desc_order.last().unwrap().id;
    assert_eq!(last_id, games_desc_order[0].id);

    let games_offset: Vec<Game> = api_wrapper.games()
      .fields("name")
      .limit("5")
      .offset("3")
      .sort_asc("id")
      .request()
      .unwrap();

    assert_eq!(7, games_offset[3].id);
  }

  #[test]
  fn json_response_test() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let test_characters: Vec<serde_json::Value> = api_wrapper.characters()
      .fields("name, gender, country_name")
      .where_like("gender != null")
      .limit("4")
      .request_json()
      .unwrap();

    let expected_result = vec![
      serde_json::json!({
        "gender": 0,
        "id": 4445,
        "name": "Beast"
      }),
      serde_json::json!({
        "gender": 0,
        "id": 8988,
        "name": "Mr. Wong"
      }),
      serde_json::json!({
        "gender": 1,
        "id": 1143,
        "name": "Annie"
      }), 
      serde_json::json!({
        "gender": 0,
        "id": 6032,
        "name": "Richtofen"
      })
    ];

    assert_eq!(&test_characters, &expected_result);
  }

  #[test]
  fn api_error() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let errored_call: Result<Vec<Genre>> = api_wrapper.genres()
      .fields("name")
      .search("Puzzle")
      .request();

    let expected_result = APIError::from_raw(
      "400".to_string(),
      "[\n  {\n    \"title\": \"Search Exception\",\n    \"status\": 400,\n    \"cause\": \"Cannot search for genre\",\n    \"details\": \"Searchable endpoints: Characters, Collections, Games, Platforms, Themes\"\n  }\n]".to_string()
    );

    assert!(&errored_call.is_err());
    assert_eq!(&expected_result, &errored_call.err().unwrap())
  }
}
