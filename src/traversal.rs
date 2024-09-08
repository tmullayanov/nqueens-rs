use std::{collections::VecDeque, marker::PhantomData};

use crate::{node::*, qboard::QBoard};

pub struct Traversal<T, Q>
where
    T: TraversalNode<Q>,
{
    queue: VecDeque<T>,
    answer: Vec<T>,
    solved: bool,
    _t: PhantomData<Q>,
}

impl Traversal<QNode, QBoard> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            answer: vec![],
            solved: false,
            _t: PhantomData,
        }
    }

    pub fn init(&mut self, node: QNode) {
        self.queue.push_back(node);
    }

    pub fn solve(&mut self) {
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

    pub fn answer(&self) -> Option<Vec<QBoard>> {
        if self.solved {
            Some(self.answer.iter().flat_map(|n| n.answer()).collect())
        } else {
            None
        }
    }
}

pub fn run(n: u8) -> Vec<QBoard> {
    let init_board = QBoard::new(n as usize);
    let node = QNode::from_board(init_board);

    let mut traversal = Traversal::new();
    traversal.init(node);
    traversal.solve();

    traversal
        .answer()
        .expect("Unexpected NONE after calling traversal.solve()")
}

#[cfg(test)]
mod tests {
    use crate::qboard::QBoard;

    use super::*;

    #[test]
    fn create_traversal() {
        let mut traversal = Traversal::new();

        let board = QBoard::new(5);
        let node = QNode::from_board(board);
        traversal.init(node);

        assert!(traversal.answer().is_none())
    }

    #[test]
    fn solve_1x1_board() {
        let mut traversal = Traversal::new();

        let board = QBoard::new(1);
        let node = QNode::from_board(board);
        traversal.init(node);
        traversal.solve();

        let answer = traversal.answer();

        assert!(answer.is_some());
        let boards = answer.unwrap();

        assert_eq!(boards.len(), 1);
        assert_eq!(boards[0].pieces(), vec![(0, 0)]);
    }

    #[test]
    fn solve_6x6_board() {
        let node = QNode::from_board(QBoard::new(6));
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
            assert_eq!(board.pieces(), au[idx]);
        }
    }

    #[test]
    fn run_with_simple_api() {
        let n = 1;

        let answer = run(n);
        assert_eq!(answer.len(), 1);
        assert_eq!(vec![(0, 0)], answer[0].pieces());
    }
}
