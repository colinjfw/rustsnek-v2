use super::super::api;
use super::minmax::Walker;
use super::*;
use log::*;
use std::convert::TryInto;
use std::time::Duration;

pub struct Runner {
    walker: Walker,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            walker: Walker::new(),
        }
    }

    pub fn run(&mut self, req: &api::MoveRequest) -> api::Direction {
        let board = self.from_request(req);
        let node = self.walker.walk(
            board,
            Options {
                max_depth: 20,
                sla: Duration::from_millis(75),
            },
        );

        debug!("board:\n{}", node.board);
        debug!("tree:\n{}", node);

        match node.pick() {
            Move::Up => api::Direction::Up,
            Move::Down => api::Direction::Down,
            Move::Left => api::Direction::Left,
            Move::Right => api::Direction::Right,
        }
    }

    fn from_request(&mut self, req: &api::MoveRequest) -> Board {
        let mut snakes = Vec::new();
        let mut food = Vec::new();
        for point in &req.board.food {
            food.push((point.x.into(), point.y.into()));
        }

        let mut my_body = Vec::with_capacity(req.you.body.len());
        for point in &req.you.body {
            my_body.push((point.x.into(), point.y.into()));
        }
        snakes.push(Snake::new(my_body));

        for snake in &req.board.snakes {
            if snake.id == req.you.id {
                continue;
            }
            let mut snake_body = Vec::with_capacity(snake.body.len());
            for point in &snake.body {
                snake_body.push((point.x.into(), point.y.into()));
            }
            snakes.push(Snake::new(snake_body));
        }
        let game = Game {
            width: req.board.width.try_into().unwrap(),
            height: req.board.height.try_into().unwrap(),
        };
        Board::new(game, snakes, food)
    }
}
