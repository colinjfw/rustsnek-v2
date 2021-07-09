use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RootResponse {
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveRequest<'a> {
    pub game: Game,
    pub turn: u16,
    pub board: Board,

    #[serde(borrow)]
    pub you: MySnake<'a>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub timeout: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub height: u16,
    pub width: u16,
    pub food: Vec<Point>,
    pub hazards: Vec<Point>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MySnake<'a> {
    pub id: &'a str,
    pub health: u16,
    pub length: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: u16,
    pub y: u16,
}
