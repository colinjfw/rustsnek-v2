use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hello {
  pub api_version: &'static str,
  pub author: &'static str,
  pub color: &'static str,
  pub head: &'static str,
  pub tail: &'static str,
  pub version: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveResponse {
  #[serde(rename = "move")]
  pub direction: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveRequest {
  pub game: Game,
  pub turn: u16,
  pub board: Board,
  pub you: Snake,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
  pub timeout: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Board {
  pub height: u16,
  pub width: u16,
  pub food: Vec<Point>,
  pub hazards: Vec<Point>,
  pub snakes: Vec<Snake>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snake {
  pub id: String,
  pub body: Vec<Point>,
  pub health: u16,
  pub length: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Point {
  pub x: u16,
  pub y: u16,
}
