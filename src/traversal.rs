use std::collections::VecDeque;

use crate::node::*;

struct Traversal {
    queue: VecDeque<Node>,
    answer: Vec<Node>,
    solved: bool,
}

impl Traversal {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            answer: vec![],
            solved: false,
        }
    }

    fn init(&mut self, node: Node) {
        self.queue.push_back(node);
    }

    fn solve(&mut self) {
        while !self.queue.is_empty() {
            let current_node = self
                .queue
                .pop_front()
                .expect("pop_front returned NONE despite !queue.is_empty()!");

            if current_node.is_leaf() {
                self.answer.push(current_node);
            } else {
                let next_gen = current_node.gen_next();
                for node in next_gen.iter().rev() {
                    if node.is_leaf() {
                        self.answer.push(node.clone());
                    } else {
                        self.queue.push_front(node.clone());
                    }
                }
            }
        }
        self.solved = true;
    }

    fn answer(&self) -> Option<Vec<Node>> {
        if self.solved {
            Some(self.answer.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::qboard::QBoard;

    use super::*;

    #[test]
    fn create_traversal() {
        let mut traversal = Traversal::new();

        let board = QBoard::new(5);
        let node = Node::from_board(board);
        traversal.init(node);

        assert!(traversal.answer().is_none())
    }

    #[test]
    fn solve_1x1_board() {
        let mut traversal = Traversal::new();

        let board = QBoard::new(1);
        let node = Node::from_board(board);
        traversal.init(node);
        traversal.solve();

        let answer = traversal.answer();

        assert!(answer.is_some());
        let boards = answer.unwrap();

        assert_eq!(boards.len(), 1);
        assert_eq!(boards[0].answer(), Some(vec![(0, 0)]));
    }

    #[test]
    fn solve_6x6_board() {
        let node = Node::from_board(QBoard::new(6));
        let mut traversal = Traversal::new();

        traversal.init(node);
        traversal.solve();

        let answer = traversal.answer();

        assert!(answer.is_some());
        let boards = answer.unwrap();
        let au = [
            [(0, 1), (1, 3), (2, 5), (3, 0), (4, 2), (5, 4)],
            [(0, 2), (1, 5), (2, 1), (3, 4), (4, 0), (5, 3)],
            [(0, 3), (1, 0), (2, 4), (3, 1), (4, 5), (5, 2)],
            [(0, 4), (1, 2), (2, 0), (3, 5), (4, 3), (5, 1)],
        ];
        for (idx, board) in boards.iter().enumerate() {
            let ans = board.answer();
            assert!(ans.is_some());

            let ans = ans.unwrap();
            assert_eq!(ans, au[idx]);
        }
    }
}
