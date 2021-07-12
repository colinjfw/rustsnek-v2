use super::*;
use std::time::Instant;

const INF: f32 = std::f32::INFINITY;

pub(super) fn walk(board: Board, opts: Options) -> Node {
    let node = Node {
        board,
        player: SnakeID(0),
        edges: Vec::with_capacity(4),
        result: Result::None,
    };
    Walker {
        opts: opts,
        start: Instant::now(),
    }
    .walk(node, SnakeID(0), 0)
}

pub(super) fn pick(node: &Node) -> Move {
    let mut min = (Move::Up, INF);
    for edge in &node.edges {
        let edge_score = score(&node, edge);
        if edge_score < min.1 {
            min = (edge.moved, edge_score);
        }
    }
    min.0
}

pub(super) fn score(source: &Node, edge: &Edge) -> f32 {
    fn cost(edge: &Edge) -> f32 {
        if edge.next.player.is_me() {
            let mut cost = match edge.next.result {
                Result::Off | Result::Dead | Result::KillMe => INF,
                Result::Eat => -20.0,
                Result::Kill => -10.0,
                Result::None => 0.0,
            };
            for next_edge in &edge.next.edges {
                if let Result::KillMe = next_edge.next.result {
                    cost += 100.0;
                }
            }
            cost
        } else {
            match edge.next.result {
                Result::Off | Result::Dead => -10.0,
                Result::None | Result::Kill | Result::Eat => 0.0,
                Result::KillMe => INF,
            }
        }
    }

    fn minimize(edge: &Edge) -> f32 {
        let node = &edge.next;
        let cost = cost(edge);
        if node.is_leaf() {
            return cost;
        }

        let mut min = INF;
        for edge in &node.edges {
            let edge_score = score(&node, edge) + cost;
            if edge_score < min {
                min = edge_score;
            }
        }
        min
    }

    fn maximize(edge: &Edge) -> f32 {
        let node = &edge.next;
        let cost = cost(edge);
        if node.is_leaf() {
            return cost;
        }

        let mut max = -INF;
        for edge in &node.edges {
            let edge_score = score(&node, edge) + cost;
            if edge_score > max {
                max = edge_score;
            }
        }
        max
    }

    if source.player.is_me() {
        minimize(edge)
    } else {
        maximize(edge)
    }
}

struct Walker {
    opts: Options,
    start: Instant,
}

impl Walker {
    fn prune(&self, node: &Node, depth: usize) -> bool {
        (node.player.is_me() && matches!(node.result, Result::Off))
            || depth >= self.opts.max_depth
            || self.start.elapsed() >= self.opts.sla
    }

    fn walk(&mut self, mut node: Node, player: SnakeID, depth: usize) -> Node {
        if self.prune(&node, depth) {
            return node;
        }
        for m in Move::all() {
            let next = play(&node.board, m, player);
            let next_edge = Edge {
                moved: m,
                next: self.walk(next, node.board.next_player(player), depth + 1),
            };
            node.edges.push(next_edge);
        }
        node
    }
}

fn play(board: &Board, m: Move, player: SnakeID) -> Node {
    let head = board.snake(player).head();
    let next_head = m.next(head);
    let mut next_board = board.clone();
    let mut next_snake = next_board.snake(player).clone();

    let result = match next_board.get(next_head) {
        Square::Off => Result::Off,
        Square::Food => {
            next_board.remove_food(next_head);
            Result::Eat
        }
        Square::Empty => {
            next_snake.body.pop();
            Result::None
        }
        Square::Snake(s) => {
            let snake = next_board.snake(s);
            if next_snake.len() > snake.len() && next_head == snake.head() {
                if s.is_me() {
                    Result::KillMe
                } else {
                    Result::Kill
                }
            } else {
                Result::Dead
            }
        }
    };

    next_snake.body.insert(0, next_head);
    next_board.set_snake(player, next_snake);
    Node {
        player,
        edges: Vec::with_capacity(4),
        result,
        board: next_board,
    }
}

#[cfg(test)]
mod test {
    use super::super::Snake;
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn basic_single_player_setup() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![(0, 0)],
            snakes: vec![Snake::new(vec![(0, 1), (0, 2)])],
        };

        let node = Node::walk(board, opts(2));
        snap("basic_single_player_setup", &node);
    }

    #[test]
    fn basic_multi_player_setup() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![(0, 0)],
            snakes: vec![
                Snake::new(vec![(0, 1), (0, 2)]),
                Snake::new(vec![(2, 2), (2, 3)]),
            ],
        };

        let node = Node::walk(board, opts(2));
        snap("basic_multi_player_setup", &node);
    }

    #[test]
    fn basic_large_player_setup() {
        let board = Board {
            width: 10,
            height: 10,
            food: vec![(0, 0)],
            snakes: vec![
                Snake::new(vec![(0, 1), (0, 2)]),
                Snake::new(vec![(2, 2), (2, 3)]),
                Snake::new(vec![(2, 4), (2, 5)]),
                Snake::new(vec![(2, 6), (2, 7)]),
            ],
        };

        let node = Node::walk(board, opts(2));
        snap("basic_large_player_setup", &node);
    }

    #[test]
    fn potential_death() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![],
            snakes: vec![
                Snake::new(vec![(1, 0)]),
                Snake::new(vec![(0, 1), (0, 2), (0, 3)]),
            ],
        };

        let node = Node::walk(board, opts(2));
        snap("potential_death", &node);
    }

    #[test]
    fn eats_food() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![(0, 0)],
            snakes: vec![Snake::new(vec![(1, 0)])],
        };

        let node = Node::walk(board, opts(2));
        snap("eats_food", &node);
    }

    #[test]
    fn trapped_myself() {
        let board = Board {
            width: 11,
            height: 11,
            food: vec![],
            snakes: vec![Snake::new(vec![
                (0, 8),
                (1, 8),
                (1, 9),
                (1, 10),
                (2, 10),
                (3, 10),
            ])],
        };

        let node = Node::walk(board, opts(3));
        snap("trapped_myself", &node);
    }

    #[test]
    fn trapped_by_another() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![],
            snakes: vec![
                Snake::new(vec![(2, 0), (2, 1)]),
                Snake::new(vec![(1, 1), (0, 1), (0, 2), (0, 3)]),
            ],
        };

        let node = Node::walk(board, opts(6));
        snap("trapped_by_another", &node);
    }

    fn opts(depth: usize) -> Options {
        Options {
            max_depth: depth,
            sla: Duration::from_secs(20),
        }
    }

    fn snap(name: &str, node: &Node) {
        let mut f = File::create("src/snake/testdata/".to_string() + name).unwrap();
        write!(f, "board:\n").unwrap();
        write!(f, "{}\n", node.board).unwrap();
        write!(f, "tree:\n").unwrap();
        write!(f, "{}", node).unwrap();

        print!("board:\n");
        print!("{}\n", node.board);
        print!("tree:\n");
        print!("{}", node);
    }
}
