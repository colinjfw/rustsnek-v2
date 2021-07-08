use super::api::{MoveRequest, Direction};

pub fn run(_req: &MoveRequest) -> Direction {
    Direction::Up
}
