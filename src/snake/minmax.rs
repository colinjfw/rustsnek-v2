use super::*;

const MAX_DEPTH: usize = 5;
const INF: f32 = std::f32::INFINITY;

pub(super) fn walk(board: Board) -> Node {
    walk_inner(Result::None, board, SnakeID(0), 0)
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
    fn minimize(edge: &Edge) -> f32 {
        let node = &edge.next;

        // Cost to my snake
        fn cost(edge: &Edge) -> f32 {
            match edge.next.result {
                Result::Off => INF,
                Result::Dead => INF,
                Result::Eat => -1.0,
                Result::None => 0.0,
            }
        }

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

        // Benefit to me of actions from another snake
        fn cost(edge: &Edge) -> f32 {
            match edge.next.result {
                Result::Off => -10.0,
                Result::Dead => -100.0,
                Result::Eat => 5.0,
                Result::None => 0.0,
            }
        }

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

fn prune(result: Result, depth: usize) -> bool {
    matches!(result, Result::Dead | Result::Off) || depth >= MAX_DEPTH
}

fn walk_inner(result: Result, board: Board, player: SnakeID, depth: usize) -> Node {
    let mut node = Node {
        board,
        edges: Vec::with_capacity(4),
        player,
        result,
    };
    if !prune(result, depth) {
        for m in [Move::Up, Move::Down, Move::Left, Move::Right] {
            let (result, next_board) = play(&node.board, m, player);
            let next_edge = Edge {
                moved: m,
                next: walk_inner(
                    result,
                    next_board,
                    node.board.next_player(player),
                    depth + 1,
                ),
            };
            node.edges.push(next_edge);
        }
    }
    node
}


fn play(board: &Board, m: Move, player: SnakeID) -> (Result, Board) {
    let head = board.snake(player).head();
    let next_head = match m {
        Move::Up => (head.0, head.1 + 1),
        Move::Down => (head.0, head.1 - 1),
        Move::Left => (head.0 - 1, head.1),
        Move::Right => (head.0 + 1, head.1),
    };
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
        Square::Me => Result::Dead,
        Square::Snake(_) => {
            Result::Dead
        },
    };

    next_snake.body.insert(0, next_head);
    next_board.set_snake(player, next_snake);
    (result, next_board)
}

#[cfg(test)]
mod test {
    use super::super::Snake;
    use super::*;
    use std::fs::File;
    use std::io::Write;

    impl Snake {
        fn me(body: Vec<Pos>) -> Snake {
            Snake { me: true, body }
        }
        fn other(body: Vec<Pos>) -> Snake {
            Snake { me: false, body }
        }
    }

    #[test]
    fn basic_single_player_setup() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![(0, 0)],
            snakes: vec![Snake::me(vec![(0, 1), (0, 2)])],
        };

        let node = Node::walk(board);
        snap("basic_single_player_setup", &node);
    }

    #[test]
    fn basic_multi_player_setup() {
        let board = Board {
            width: 5,
            height: 5,
            food: vec![(0, 0)],
            snakes: vec![
                Snake::me(vec![(0, 1), (0, 2)]),
                Snake::other(vec![(2, 2), (2, 3)]),
            ],
        };

        let node = Node::walk(board);
        snap("basic_multi_player_setup", &node);
    }

    #[test]
    fn basic_large_player_setup() {
        let board = Board {
            width: 10,
            height: 10,
            food: vec![(0, 0)],
            snakes: vec![
                Snake::me(vec![(0, 1), (0, 2)]),
                Snake::other(vec![(2, 2), (2, 3)]),
                Snake::other(vec![(2, 4), (2, 5)]),
                Snake::other(vec![(2, 6), (2, 7)]),
            ],
        };

        let node = Node::walk(board);
        snap("basic_large_player_setup", &node);
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
