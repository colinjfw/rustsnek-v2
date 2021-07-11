mod api;
mod minmax;

pub use api::run;
use std::fmt;

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Result {
    None,
    Eat,
    Dead,
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
enum Square {
    Me,
    Snake(usize),
    Food,
    Empty,
    Off,
}

#[derive(Clone, Debug)]
struct Snake {
    me: bool,
    body: Vec<Pos>,
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
                    return if snake.me { Square::Me } else { Square::Snake(i) };
                }
            }
        }
        Square::Empty
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
    player: usize,
    result: Result,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.edges.is_empty()
    }

    fn is_me(&self) -> bool {
        self.player == 0
    }

    fn walk(board: Board) -> Node {
        minmax::walk(board)
    }

    fn pick(&self) -> Move {
        minmax::pick(self)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            write!(f, "  ")?;
            for x in 0..self.width {
                match self.get((x, y)) {
                    Square::Me => write!(f, "M ")?,
                    Square::Snake(i) => write!(f, "{} ", i)?,
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
                        child.next.player,
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
            format!("Root [{}] {}", self.player, self.pick()),
            "".to_string(),
            true,
        )
    }
}
