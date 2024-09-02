#[derive(Clone, Debug)]
pub struct QBoard {
    pub pieces_placed: u8,
    size: usize,
    pieces: Vec<Option<usize>>,
}

impl QBoard {
    pub fn new(size: usize) -> Self {
        let mut pieces = Vec::with_capacity(size);
        for _ in 0..size {
            pieces.push(None);
        }

        Self {size, pieces_placed: 0, pieces: pieces}
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_piece(&mut self, row: usize, col: usize) {
        if col >= self.size || row >= self.size {
            panic!("Setting piece at {row}:{col} which is outside of board bounds (size={})", self.size);
        }
        self.pieces[row] = Some(col);
        self.pieces_placed += 1;
    }

    pub fn pieces(&self) -> Vec<(usize, usize)> {
        self.pieces
            .iter()
            .flatten()
            .enumerate()
            .map(|(col, row)| (col, *row))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        assert!(true);
    }

    #[test]
    fn create_board() {
        let n: usize = 5;
        let b = QBoard::new(n);

        assert_eq!(b.size(), n);
    }

    #[test]
    fn set_piece() {
        let mut b = QBoard::new(5);
        b.set_piece(0, 0);

        assert_eq!(b.pieces(), vec![(0, 0)]);
    }

    #[test]
    #[should_panic]
    fn panics_when_placing_pieces_outside_of_the_board() {
        let mut board = QBoard::new(5);

        board.set_piece(0, 10);
    }

}