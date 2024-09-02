use crate::qboard::*;

pub struct Node {
    generation: u8,
    board: QBoard,
}

impl Node {
    fn new(board: QBoard) -> Self {
        Node {
            generation: board.pieces().len().try_into().unwrap(),
            board: board
        }
    }

    fn is_valid(&self) -> bool {
        let pieces = self.board.pieces();

        for i in 0..pieces.len()-1 {
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

    fn gen_next(&self) -> Vec<QBoard> {
        let next_row = self.board.pieces_placed;
        let mut correct_boards = vec![];

        for col in 0..self.board.size() {
            let next_pt: (usize, usize) = (next_row.into(), col);

            let active_pieces = self.board.pieces();
            active_pieces
                .iter()
                .all(|x| {
                    !self.see_each_other(*x, next_pt)
                })
                .then(|| {
                    let mut next_board = self.board.clone();
                    next_board.set_piece(next_pt.0, next_pt.1);
                    correct_boards.push(next_board);
                });

        }

        correct_boards
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
        let checker = Node::new(b);

        assert!(!checker.is_valid());

        let mut b = QBoard::new(2);
        b.set_piece(0, 0);
        b.set_piece(1, 1);
        assert!(!checker.is_valid());

        let mut b = QBoard::new(3);
        b.set_piece(1, 1);
        b.set_piece(1, 2);
        assert!(!checker.is_valid());
    }

    #[test]
    fn generate_next_variants() {
        let mut b = QBoard::new(5);
        b.set_piece(0, 0);

        let node = Node::new(b);

        let next_boards = node.gen_next();

        assert_eq!(next_boards.len(), 3);
        for board in &next_boards {
            assert_eq!(board.pieces_placed, 2);
        } 
    }

    #[test]
    fn generate_boards_from_scratch() {
        let b = QBoard::new(5);

        let node = Node::new(b);

        let next_boards = node.gen_next();

        assert_eq!(next_boards.len(), 5);
        for (idx, board) in next_boards.iter().enumerate() {
            assert_eq!(board.pieces_placed, 1);
            
            let pieces = board.pieces();
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

        let node = Node::new(b);
        let boards = node.gen_next();

        assert_eq!(boards.len(), 0);
    }
}