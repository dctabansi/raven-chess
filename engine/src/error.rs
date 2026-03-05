#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenError {
    MissingField(&'static str),
    InvalidRanksCount(usize),
    InvalidEmptySquares(u8),
    TooManyFilesInRank(u32),
    InvalidCharacter(char),
    InvalidColor(String),
    InvalidCastlingCharacter(char),
    InvalidEnPassantTarget(String),
    InvalidHalfMoveClock(String),
    InvalidFullMoveNumber(String),
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenError::MissingField(field) => {
                write!(f, "Missing FEN field: {}", field)
            }
            FenError::InvalidRanksCount(count) => {
                write!(f, "Expected 8 ranks, found {}", count)
            }
            FenError::InvalidEmptySquares(val) => {
                write!(f, "Invalid empty squares count: {}", val)
            }
            FenError::TooManyFilesInRank(rank) => {
                write!(f, "Rank {} has more than 8 files", rank)
            }
            FenError::InvalidCharacter(c) => {
                write!(f, "Invalid board character: '{}'", c)
            }
            FenError::InvalidColor(c) => {
                write!(f, "Invalid active color: '{}'", c)
            }
            FenError::InvalidCastlingCharacter(c) => {
                write!(f, "Invalid castling character: '{}'", c)
            }
            FenError::InvalidEnPassantTarget(sq) => {
                write!(f, "Invalid en passant target: '{}'", sq)
            }
            FenError::InvalidHalfMoveClock(clk) => {
                write!(f, "Invalid half-move clock: '{}'", clk)
            }
            FenError::InvalidFullMoveNumber(num) => {
                write!(f, "Invalid full-move number: '{}'", num)
            }
        }
    }
}

impl std::error::Error for FenError {}
