use std::fmt::Display;

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

        Self {
            size,
            pieces_placed: 0,
            pieces: pieces,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_piece(&mut self, row: usize, col: usize) {
        if col >= self.size || row >= self.size {
            panic!(
                "Setting piece at {row}:{col} which is outside of board bounds (size={})",
                self.size
            );
        }
        self.pieces[row] = Some(col);
        self.pieces_placed += 1;
    }

    pub fn pieces(&self) -> Vec<(usize, usize)> {
        self.pieces
            .iter()
            .enumerate()
            .filter(|(_, opt)| opt.is_some())
            .map(|(col, row)| (col, row.unwrap()))
            .collect()
    }
}

impl Display for QBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", "#".repeat(self.size + 2))?;

        for pos in self.pieces.iter().rev() {
            let line: String = match pos {
                Some(pos) => (0..self.size)
                    .map(|x| if x == *pos { '*' } else { '.' })
                    .collect(),
                None => ".".repeat(self.size),
            };
            write!(f, "{}\n", line)?;
        }

        write!(f, "{}\n\n", "#".repeat(self.size + 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn pieces_dont_lose_rows_when_set_insequentally() {
        let mut b = QBoard::new(5);
        b.set_piece(0, 0);
        b.set_piece(2, 1);

        assert_eq!(b.pieces(), [(0, 0), (2, 1)]);
    }

    #[test]
    fn qboard_impl_display() {
        let mut b = QBoard::new(2);
        b.set_piece(0, 0);
        b.set_piece(1, 1);

        // the output should look like this:
        // ####
        // #.*#
        // #*.#
        // ####
        assert_eq!(b.to_string(), "####\n.*\n*.\n####\n\n");
    }
}
