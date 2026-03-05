use crate::error::FenError;
use crate::types::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub white_occupancy: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
    pub black_occupancy: u64,

    pub all_occupancy: u64,

    pub active_color: Color,

    pub castling_rights: u8,

    pub en_passant_target: Option<u8>,

    pub half_move_clock: u8,
    pub full_move_number: u16,
}

impl Board {
    pub fn new() -> Self {
        Board {
            // White Pieces (Ranks 1 and 2)
            white_pawns: 0x0000_0000_0000_FF00,
            white_knights: 0x0000_0000_0000_0042,
            white_bishops: 0x0000_0000_0000_0024,
            white_rooks: 0x0000_0000_0000_0081,
            white_queens: 0x0000_0000_0000_0008,
            white_king: 0x0000_0000_0000_0010,
            white_occupancy: 0x0000_0000_0000_FFFF,

            // Black Pieces (Ranks 7 and 8)
            black_pawns: 0x00FF_0000_0000_0000,
            black_knights: 0x4200_0000_0000_0000,
            black_bishops: 0x2400_0000_0000_0000,
            black_rooks: 0x8100_0000_0000_0000,
            black_queens: 0x0800_0000_0000_0000,
            black_king: 0x1000_0000_0000_0000,
            black_occupancy: 0xFFFF_0000_0000_0000,

            all_occupancy: 0xFFFF_0000_0000_FFFF,

            active_color: Color::White,

            // 0b1111 represents KQkq (W King-side, W Queen-side, B King-side, B Queen-side)
            castling_rights: 0b1111,

            en_passant_target: None,

            half_move_clock: 0,
            full_move_number: 1,
        }
    }

    pub fn empty() -> Self {
        Board {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            white_occupancy: 0,

            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,
            black_occupancy: 0,

            all_occupancy: 0,

            active_color: Color::White,

            castling_rights: 0,

            en_passant_target: None,

            half_move_clock: 0,

            full_move_number: 1,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        fen.parse()
    }

    pub fn is_legal(&self) -> bool {
        // One king per side
        if self.white_king.count_ones() != 1 || self.black_king.count_ones() != 1 {
            return false;
        }

        // No pawns on ranks 1 or 8
        let rank_1_8_mask: u64 = 0xFF00_0000_0000_00FF;
        if (self.white_pawns & rank_1_8_mask) != 0 || (self.black_pawns & rank_1_8_mask) != 0 {
            return false;
        }

        true
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl std::str::FromStr for Board {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::empty();

        // Zero-allocation field extraction
        let mut fen_fields = s.split_ascii_whitespace();
        let board_str = fen_fields.next().ok_or(FenError::MissingField("board"))?;
        let color_str = fen_fields.next().ok_or(FenError::MissingField("color"))?;
        let castling_str = fen_fields.next().ok_or(FenError::MissingField("castling"))?;
        let en_passant_str = fen_fields.next().ok_or(FenError::MissingField("en_passant"))?;
        let half_move_str = fen_fields.next().ok_or(FenError::MissingField("half_move"))?;
        let full_move_str = fen_fields.next().ok_or(FenError::MissingField("full_move"))?;

        // Set piece positions
        let rank_strings = board_str.split('/');

        let ranks_count = rank_strings.clone().count();
        if ranks_count != 8 {
            return Err(FenError::InvalidRanksCount(ranks_count));
        }

        for (i, rank_str) in rank_strings.enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let rank_index = 7 - (i as u32);
            let mut file_index = 0;

            for byte in rank_str.bytes() {
                if byte.is_ascii_digit() {
                    let empty_squares = u32::from(byte - b'0');

                    if !(1..=8).contains(&empty_squares) {
                        return Err(FenError::InvalidEmptySquares(empty_squares as u8));
                    }

                    file_index += empty_squares;

                    if file_index > 8 {
                        let rank = rank_index + 1;
                        return Err(FenError::TooManyFilesInRank(rank));
                    }

                    continue;
                }

                if byte.is_ascii_alphabetic() {
                    if file_index >= 8 {
                        let rank = rank_index + 1;
                        return Err(FenError::TooManyFilesInRank(rank));
                    }

                    let square_index = rank_index * 8 + file_index;
                    match byte {
                        b'P' => board.white_pawns |= 1u64 << square_index,
                        b'p' => board.black_pawns |= 1u64 << square_index,
                        b'N' => board.white_knights |= 1u64 << square_index,
                        b'n' => board.black_knights |= 1u64 << square_index,
                        b'B' => board.white_bishops |= 1u64 << square_index,
                        b'b' => board.black_bishops |= 1u64 << square_index,
                        b'R' => board.white_rooks |= 1u64 << square_index,
                        b'r' => board.black_rooks |= 1u64 << square_index,
                        b'Q' => board.white_queens |= 1u64 << square_index,
                        b'q' => board.black_queens |= 1u64 << square_index,
                        b'K' => board.white_king |= 1u64 << square_index,
                        b'k' => board.black_king |= 1u64 << square_index,
                        _ => return Err(FenError::InvalidCharacter(byte as char)),
                    }
                    file_index += 1;
                    continue;
                }

                return Err(FenError::InvalidCharacter(byte as char));
            }

            if file_index != 8 {
                let rank = rank_index + 1;
                return Err(FenError::TooManyFilesInRank(rank));
            }
        }

        // Set active color
        match color_str {
            "w" => board.active_color = Color::White,
            "b" => board.active_color = Color::Black,
            _ => return Err(FenError::InvalidColor(color_str.to_string())),
        }

        // Set castling rights
        if castling_str == "-" {
            board.castling_rights = 0;
        } else {
            for byte in castling_str.bytes() {
                match byte {
                    b'-' => break,
                    b'K' => board.castling_rights |= 0b0001,
                    b'Q' => board.castling_rights |= 0b0010,
                    b'k' => board.castling_rights |= 0b0100,
                    b'q' => board.castling_rights |= 0b1000,
                    _ => return Err(FenError::InvalidCastlingCharacter(byte as char)),
                }
            }
        }

        // Set en passant target square
        if en_passant_str != "-" {
            let bytes = en_passant_str.as_bytes();

            if bytes.len() != 2 {
                return Err(FenError::InvalidEnPassantTarget(en_passant_str.to_string()));
            }

            let file_char = bytes[0];
            let rank_char = bytes[1];

            if !(b'a'..=b'h').contains(&file_char) || !(b'1'..=b'8').contains(&rank_char) {
                return Err(FenError::InvalidEnPassantTarget(en_passant_str.to_string()));
            }

            let rank = rank_char - b'1';
            let file = file_char - b'a';
            board.en_passant_target = Some(rank * 8 + file);
        }

        // Set the half-move clock
        board.half_move_clock = half_move_str
            .parse::<u8>()
            .map_err(|_| FenError::InvalidHalfMoveClock(half_move_str.to_string()))?;

        // Set the full-move number
        board.full_move_number = full_move_str
            .parse::<u16>()
            .map_err(|_| FenError::InvalidFullMoveNumber(full_move_str.to_string()))?;

        // Calculate occupancies
        board.white_occupancy = board.white_pawns
            | board.white_knights
            | board.white_bishops
            | board.white_rooks
            | board.white_queens
            | board.white_king;

        board.black_occupancy = board.black_pawns
            | board.black_knights
            | board.black_bishops
            | board.black_rooks
            | board.black_queens
            | board.black_king;

        board.all_occupancy = board.white_occupancy | board.black_occupancy;

        // Return the board
        Ok(board)
    }
}
