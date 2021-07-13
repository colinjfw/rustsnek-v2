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

#[test]
fn trapped_game_regression_1() {
    let board = Board {
        width: 11,
        height: 11,
        food: vec![(6, 6)],
        snakes: vec![
            Snake::new(vec![(7, 6), (7, 5), (7, 4)]),
            Snake::new(vec![(5, 7), (5, 6), (5, 5), (5, 4), (5, 3), (6, 3), (7, 3), (8, 3)]),
        ],
    };

    let node = Node::walk(board, opts(3));
    snap("trapped_game_regression_1", &node);
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
