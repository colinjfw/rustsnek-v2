use super::*;
use std::time::Instant;

const INF: f32 = std::f32::INFINITY;

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

fn cost(edge: &Edge) -> f32 {
    fn will_be_killed_factor(edge: &Edge) -> f32 {
        let mut factor = 0.0;
        for next_edge in &edge.next.edges {
            if let Result::KillMe = next_edge.next.result {
                factor += 4.0;
            }
        }
        factor
    }
    fn escapability_factor(edge: &Edge) -> f32 {
        let mut factor = 0.0;
        for next_edge in &edge.next.edges {
            if next_edge.next.player.is_me()
                && matches!(next_edge.next.result, Result::Dead | Result::Off)
            {
                factor += 1.0;
            }
        }
        factor
    }

    let mut cost = if edge.next.player.is_me() {
        match edge.next.result {
            Result::Off | Result::Dead | Result::KillMe => INF,
            Result::Eat => -4.0,
            Result::Kill => -1.0,
            Result::None => 0.0,
        }
    } else {
        match edge.next.result {
            Result::Off | Result::Dead => -1.0,
            Result::None | Result::Kill | Result::Eat => 0.0,
            Result::KillMe => INF,
        }
    };
    cost += will_be_killed_factor(edge);
    cost += escapability_factor(edge);
    cost
}

pub(super) fn score(source: &Node, edge: &Edge) -> f32 {
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

pub(super) struct Walker {
    opts: Options,
    start: Instant,
}

impl Walker {
    pub(super) fn new() -> Walker {
        Walker{
            opts: Options{ max_depth: 0, sla: Duration::from_secs(0) },
            start: Instant::now(),
        }
    }

    pub(super) fn walk(&mut self, board: Board, opts: Options) -> Node {
        let node = Node {
            board,
            player: SnakeID(0),
            edges: Vec::with_capacity(4),
            result: Result::None,
        };
        self.opts = opts;
        self.start = Instant::now();
        self.walk_node(node, SnakeID(0), 0)
    }

    fn prune(&self, node: &Node, depth: usize) -> bool {
        (node.player.is_me() && matches!(node.result, Result::Off))
            || depth >= self.opts.max_depth
            || self.start.elapsed() >= self.opts.sla
    }

    fn walk_node(&mut self, mut node: Node, player: SnakeID, depth: usize) -> Node {
        if self.prune(&node, depth) {
            return node;
        }
        for m in Move::all() {
            let next = self.play(&node.board, m, player);
            let next_edge = Edge {
                moved: m,
                next: self.walk_node(next, node.board.next_player(player), depth + 1),
            };
            node.edges.push(next_edge);
        }
        node
    }

    fn play(&mut self, board: &Board, m: Move, player: SnakeID) -> Node {
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
                next_snake.remove_tail();
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

        next_snake.change_head(next_head);
        next_board.set_snake(player, next_snake);
        Node {
            player,
            edges: Vec::with_capacity(4),
            result,
            board: next_board,
        }
    }
}
