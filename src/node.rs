use crate::qboard::*;

#[derive(Clone, Debug)]
pub struct Node {
    board: QBoard,
}

impl Node {
    pub fn from_board(board: QBoard) -> Self {
        Node { board: board }
    }

    pub fn is_valid(&self) -> bool {
        let pieces = self.board.pieces();

        for i in 0..pieces.len() - 1 {
            for j in i..pieces.len() {
                if self.see_each_other(pieces[i], pieces[j]) {
                    return false;
                }
            }
        }

        true
    }

    fn see_each_other(&self, lhs: (usize, usize), rhs: (usize, usize)) -> bool {
        let (lhs_x, lhs_y) = lhs;
        let (rhs_x, rhs_y) = rhs;

        let horizontal = lhs_x == rhs_x;
        let vertical = lhs_y == rhs_y;
        let diagonal = lhs_x.abs_diff(rhs_x) == lhs_y.abs_diff(rhs_y);

        return horizontal || vertical || diagonal;
    }

    pub fn gen_next(&self) -> Vec<Node> {
        let next_row = self.board.pieces_placed;
        let mut next_get_nodes = vec![]; // only correct boards are pushed

        for col in 0..self.board.size() {
            let next_pt: (usize, usize) = (next_row.into(), col);

            let active_pieces = self.board.pieces();
            active_pieces
                .iter()
                .all(|x| !self.see_each_other(*x, next_pt))
                .then(|| {
                    let mut next_board = self.board.clone();
                    next_board.set_piece(next_pt.0, next_pt.1);

                    let node = Node::from_board(next_board);
                    next_get_nodes.push(node);
                });
        }

        next_get_nodes
    }

    pub fn is_leaf(&self) -> bool {
        self.board.size() == self.board.pieces_placed.into()
    }

    pub fn answer(&self) -> Option<Vec<(usize, usize)>> {
        if self.is_leaf() {
            Some(self.board.pieces())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_validate_board() {
        let mut b = QBoard::new(5);
        b.set_piece(0, 0);
        b.set_piece(1, 0);
        let checker = Node::from_board(b);

        assert!(!checker.is_valid());

        let mut b = QBoard::new(2);
        b.set_piece(0, 0);
        b.set_piece(1, 1);
        assert!(!checker.is_valid());

        let mut b = QBoard::new(3);
        b.set_piece(1, 1);
        b.set_piece(1, 2);
        assert!(!checker.is_valid());

        let mut b = QBoard::new(5);
        b.set_piece(0, 0);
        b.set_piece(2, 1);
        assert!(checker.is_valid());
    }

    #[test]
    fn generate_next_variants() {
        let mut b = QBoard::new(5);
        b.set_piece(0, 0);

        let node = Node::from_board(b);

        let next_boards = node.gen_next();

        assert_eq!(next_boards.len(), 3);
        for node in &next_boards {
            assert_eq!(node.board.pieces_placed, 2);
        }
    }

    #[test]
    fn generate_nodes_from_scratch() {
        let b = QBoard::new(5);

        let node = Node::from_board(b);

        let next_nodes = node.gen_next();

        assert_eq!(next_nodes.len(), 5);
        for (idx, node) in next_nodes.iter().enumerate() {
            assert_eq!(node.board.pieces_placed, 1);

            let pieces = node.board.pieces();
            assert_eq!(pieces.len(), 1);
            let (row, col) = pieces[0];

            assert_eq!(row, 0);
            assert_eq!(col, idx);
        }
    }

    #[test]
    fn generate_nothing_from_last_gen() {
        let mut b = QBoard::new(1);
        b.set_piece(0, 0);

        let node = Node::from_board(b);
        let boards = node.gen_next();

        assert_eq!(boards.len(), 0);
    }

    #[test]
    fn is_leaf() {
        let mut b = QBoard::new(1);
        b.set_piece(0, 0);

        let node = Node::from_board(b);

        assert!(node.is_leaf());

        let mut b = QBoard::new(2);
        b.set_piece(0, 0);
        let node = Node::from_board(b);

        assert!(!node.is_leaf());
    }

    #[test]
    fn gives_answer_on_leafs() {
        let mut b1 = QBoard::new(5);
        let node1 = Node::from_board(b1);
        assert!(node1.answer().is_none());

        let mut b2 = QBoard::new(1);
        b2.set_piece(0, 0);
        let node2 = Node::from_board(b2);

        assert!(node2.answer().is_some());
        let ans = node2.answer().unwrap();

        assert_eq!(ans, vec![(0, 0)]);
    }
}
