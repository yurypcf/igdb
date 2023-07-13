use crate::{
    models::*,
    utils::response_handler::{APIError, Result},
};
use pretty_assertions::assert_eq;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

struct EndpointUtilsTest;

trait EndpointUtilsTestRequestor {
    type VecType;

    fn request() -> Result<Vec<Self::VecType>>;
    fn request_error() -> Result<Vec<Self::VecType>>;
}

impl EndpointUtilsTestRequestor for EndpointUtilsTest {
    type VecType = Game;

    fn request() -> Result<Vec<Self::VecType>> {
        let mut root_path = root_path();
        root_path.push("src/tests/resources/game_response_test.txt");

        let data = read_to_string(root_path).unwrap();

        let resp: Vec<Game> = serde_json::from_str(&data).unwrap();

        Ok(resp)
    }

    fn request_error() -> Result<Vec<Self::VecType>> {
        let data = r#"invalid data"#;

        match serde_json::from_str(data) {
            Ok(resp) => Ok(resp),
            Err(err) => Err(APIError::from(err)),
        }
    }
}

fn root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

// Testing FIELDS apicalypse query
#[test]
fn game_response_test() {
    let game = EndpointUtilsTest::request().unwrap();

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

// Testing API Error being treated to consumer
#[test]
fn api_error() {
    let errored_call = EndpointUtilsTest::request_error().unwrap_err();

    let expected_result = APIError::from_raw(
        "HttpClientError".to_string(),
        "Sort options parse error: expected value at line 1 column 1".to_string(),
    );

    assert_eq!(&expected_result, &errored_call)
}
