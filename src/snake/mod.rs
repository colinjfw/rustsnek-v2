mod api;
mod minmax;

pub use api::run;
use std::fmt;
use std::time::Duration;

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn all() -> [Move; 4] {
        [Move::Up, Move::Down, Move::Left, Move::Right]
    }

    fn next(&self, pos: Pos) -> Pos {
        match self {
            Move::Up => (pos.0, pos.1 + 1),
            Move::Down => (pos.0, pos.1 - 1),
            Move::Left => (pos.0 - 1, pos.1),
            Move::Right => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Result {
    None,
    Eat,
    Dead,
    Kill,
    KillMe,
    Off,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Up => write!(f, "up"),
            Move::Down => write!(f, "down"),
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SnakeID(usize);

impl SnakeID {
    fn is_me(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone, Copy, Debug)]
enum Square {
    Snake(SnakeID),
    Food,
    Empty,
    Off,
}

#[derive(Clone, Debug)]
struct Snake {
    body: Vec<Pos>,
}

impl Snake {
    fn head(&self) -> Pos {
        self.body[0]
    }

    fn len(&self) -> usize {
        self.body.len()
    }

    fn new(body: Vec<Pos>) -> Snake {
        Snake { body }
    }
}

#[derive(Clone, Debug)]
struct Board {
    width: isize,
    height: isize,
    snakes: Vec<Snake>,
    food: Vec<Pos>,
}

impl Board {
    fn remove_food(&mut self, pos: Pos) {
        if let Some(idx) = self.food.iter().position(|food| *food == pos) {
            self.food.remove(idx);
        }
    }

    fn get(&self, pos: Pos) -> Square {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.width || pos.1 >= self.height {
            return Square::Off;
        };
        for food in &self.food {
            if *food == pos {
                return Square::Food;
            };
        }
        for (i, snake) in self.snakes.iter().enumerate() {
            for point in &snake.body {
                if *point == pos {
                    return Square::Snake(SnakeID(i));
                }
            }
        }
        Square::Empty
    }

    fn snake(&self, player: SnakeID) -> &Snake {
        &self.snakes[player.0]
    }

    fn set_snake(&mut self, player: SnakeID, snake: Snake) {
        self.snakes[player.0] = snake;
    }

    fn next_player(&self, player: SnakeID) -> SnakeID {
        if player.0 + 1 >= self.snakes.len() {
            return SnakeID(0);
        }
        return SnakeID(player.0 + 1);
    }
}

#[derive(Clone, Debug)]
struct Edge {
    next: Node,
    moved: Move,
}

#[derive(Clone, Debug)]
struct Node {
    board: Board,
    edges: Vec<Edge>,
    player: SnakeID,
    result: Result,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.edges.is_empty()
    }

    fn walk(board: Board, opts: Options) -> Node {
        minmax::walk(board, opts)
    }

    fn pick(&self) -> Move {
        minmax::pick(self)
    }
}

struct Options {
    max_depth: usize,
    sla: Duration,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            let y = self.height - row - 1;
            write!(f, "  ")?;
            for x in 0..self.width {
                match self.get((x, y)) {
                    Square::Snake(i) => write!(f, "{} ", i.0)?,
                    Square::Food => write!(f, "F ")?,
                    Square::Empty => write!(f, "_ ")?,
                    Square::Off => unreachable!(),
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn pprint_tree(
            f: &mut fmt::Formatter,
            node: &Node,
            name: String,
            prefix: String,
            last: bool,
        ) -> fmt::Result {
            let prefix_current = "|- ";

            write!(f, "{}{}{}\n", prefix, prefix_current, name)?;

            let prefix_child = if last { "   " } else { "|  " };
            let prefix = prefix + prefix_child;

            if !node.edges.is_empty() {
                let last_child = node.edges.len() - 1;

                for (i, child) in node.edges.iter().enumerate() {
                    let name = format!(
                        "{} [{}] - {}",
                        child.moved,
                        node.player.0,
                        minmax::score(node, child)
                    );
                    pprint_tree(f, &child.next, name, prefix.to_string(), i == last_child)?;
                }
            }
            Ok(())
        }

        pprint_tree(
            f,
            self,
            format!("Root [{}] {}", self.player.0, self.pick()),
            "".to_string(),
            true,
        )
    }
}
