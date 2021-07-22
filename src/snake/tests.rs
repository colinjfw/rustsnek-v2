extern crate test;

use super::minmax::Walker;
use super::*;
use std::fs::File;
use std::io::Write;
use test::Bencher;

struct Scenario {
    name: &'static str,
    width: isize,
    height: isize,
    food: Vec<Pos>,
    snakes: Vec<Snake>,
    depth: usize,
}

impl Scenario {
    fn snap(&self, node: &Node) {
        let mut f = File::create("src/snake/testdata/".to_string() + self.name).unwrap();
        write!(f, "board:\n").unwrap();
        write!(f, "{}\n", node.board).unwrap();
        write!(f, "tree:\n").unwrap();
        write!(f, "{}", node).unwrap();

        print!("board:\n");
        print!("{}\n", node.board);
        print!("tree:\n");
        print!("{}", node);
    }

    fn prepare(&self) -> (Board, Options) {
        let board = Board {
            game: Game {
                width: self.width,
                height: self.height,
            },
            food: self.food.clone(),
            snakes: self.snakes.clone(),
        };
        let opts = Options {
            max_depth: self.depth,
            sla: Duration::from_secs(20),
        };
        (board, opts)
    }
}

fn run_test(s: Scenario) {
    let (board, opts) = s.prepare();
    let node = Walker::new().walk(board, opts);
    s.snap(&node);
}

fn run_bench(b: &mut Bencher, s: Scenario) {
    b.iter(|| {
        let (board, opts) = s.prepare();
        Walker::new().walk(board, opts)
    });
}

#[test]
fn basic_single_player_setup() {
    run_test(Scenario {
        name: "basic_single_player_setup",
        width: 5,
        height: 5,
        food: vec![(0, 0)],
        snakes: vec![Snake::new(vec![(0, 1), (0, 2)])],
        depth: 2,
    })
}

#[test]
fn basic_multi_player_setup() {
    run_test(Scenario {
        name: "basic_multi_player_setup",
        width: 5,
        height: 5,
        food: vec![(0, 0)],
        snakes: vec![
            Snake::new(vec![(0, 1), (0, 2)]),
            Snake::new(vec![(2, 2), (2, 3)]),
        ],
        depth: 2,
    })
}

#[test]
fn basic_large_player_setup() {
    run_test(Scenario {
        name: "basic_large_player_setup",
        height: 10,
        width: 10,
        food: vec![(0, 0)],
        snakes: vec![
            Snake::new(vec![(0, 1), (0, 2)]),
            Snake::new(vec![(2, 2), (2, 3)]),
            Snake::new(vec![(2, 4), (2, 5)]),
            Snake::new(vec![(2, 6), (2, 7)]),
        ],
        depth: 2,
    });
}

#[test]
fn potential_death() {
    run_test(Scenario {
        name: "potential_death",
        height: 5,
        width: 5,
        food: vec![],
        snakes: vec![
            Snake::new(vec![(1, 0)]),
            Snake::new(vec![(0, 1), (0, 2), (0, 3)]),
        ],
        depth: 2,
    });
}

#[test]
fn eats_food() {
    run_test(Scenario {
        name: "eats_food",
        height: 5,
        width: 5,
        food: vec![(0, 0)],
        snakes: vec![Snake::new(vec![(1, 0)])],
        depth: 2,
    });
}

#[test]
fn trapped_myself() {
    run_test(Scenario {
        name: "trapped_myself",
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
        depth: 3,
    });
}

#[test]
fn trapped_by_another() {
    run_test(Scenario {
        name: "trapped_by_another",
        width: 5,
        height: 5,
        food: vec![],
        snakes: vec![
            Snake::new(vec![(2, 0), (2, 1)]),
            Snake::new(vec![(1, 1), (0, 1), (0, 2), (0, 3)]),
        ],
        depth: 6,
    });
}

#[test]
fn trapped_game_regression_1() {
    run_test(Scenario {
        name: "trapped_game_regression_1",
        width: 11,
        height: 11,
        food: vec![(6, 6)],
        snakes: vec![
            Snake::new(vec![(7, 6), (7, 5), (7, 4)]),
            Snake::new(vec![
                (5, 7),
                (5, 6),
                (5, 5),
                (5, 4),
                (5, 3),
                (6, 3),
                (7, 3),
                (8, 3),
            ]),
        ],
        depth: 3,
    });
}

#[bench]
fn bench_trapped(b: &mut Bencher) {
    run_bench(b, Scenario {
        name: "trapped_game_regression_1",
        width: 11,
        height: 11,
        food: vec![(6, 6)],
        snakes: vec![
            Snake::new(vec![(7, 6), (7, 5), (7, 4)]),
            Snake::new(vec![(5, 7), (5, 6), (5, 5), (5, 4), (5, 3), (6, 3), (7, 3), (8, 3)]),
        ],
        depth: 3,
    });
}

#[bench]
fn bench_basic(b: &mut Bencher) {
    run_bench(b, Scenario {
        name: "basic_single_player_setup",
        width: 5,
        height: 5,
        food: vec![(0, 0)],
        snakes: vec![Snake::new(vec![(0, 1), (0, 2)])],
        depth: 3,
    });
}
