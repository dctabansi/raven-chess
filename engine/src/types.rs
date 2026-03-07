#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub source: u8,
    pub target: u8,
    pub promotion: Option<PieceType>,
}

impl Move {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            source: 0,
            target: 0,
            promotion: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveList {
    pub moves: [Move; 256],
    pub count: usize,
}

impl MoveList {
    #[must_use]
    pub fn new() -> Self {
        Self {
            moves: [Move::empty(); 256],
            count: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BitboardIterator(pub u64);

impl Iterator for BitboardIterator {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            #[allow(clippy::cast_possible_truncation)]
            let square = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Some(square)
        }
    }
}
