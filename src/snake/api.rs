use super::super::api;
use super::*;
use log::*;
use std::convert::TryInto;

fn from_request(req: &api::MoveRequest) -> Board {
    let mut board = Board {
        width: req.board.width.try_into().unwrap(),
        height: req.board.height.try_into().unwrap(),
        snakes: Vec::new(),
        food: Vec::new(),
    };
    for point in &req.board.food {
        board.food.push((point.x.into(), point.y.into()));
    }
    for snake in &req.board.snakes {
        let mut snake_body = Vec::with_capacity(snake.body.len());
        for point in &snake.body {
            snake_body.push((point.x.into(), point.y.into()));
        }
        board.snakes.push(Snake {
            me: snake.id == req.you.id,
            body: snake_body,
        });
    }
    board
}

pub fn run(req: &api::MoveRequest) -> api::Direction {
    let node = Node::walk(from_request(req));

    debug!("board:\n{}", node.board);
    debug!("tree:\n{}", node);

    match node.pick() {
        Move::Up => api::Direction::Up,
        Move::Down => api::Direction::Down,
        Move::Left => api::Direction::Left,
        Move::Right => api::Direction::Right,
    }
}
