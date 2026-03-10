use crate::attacks::{get_bishop_attacks, get_king_attacks, get_knight_attacks, get_rook_attacks};
use crate::constants::{A_FILE, CASTLING_RIGHTS_UPDATE, H_FILE};
use crate::error::FenError;
use crate::types::{Color, Move, PieceType};

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

    pub en_passant_target: u64,

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

            en_passant_target: 0,

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

            en_passant_target: 0,

            half_move_clock: 0,

            full_move_number: 1,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        fen.parse()
    }

    pub fn to_fen(&self) -> String {
        // A standard FEN string is typically around 60-90 characters.
        let mut fen = String::with_capacity(90);

        // 1. Piece placement
        // FEN starts from Rank 8 (index 7) down to Rank 1 (index 0)
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let square_index = rank * 8 + file;
                let bit = 1u64 << square_index;

                let piece_char = if (self.white_pawns & bit) != 0 { Some('P') }
                else if (self.black_pawns & bit) != 0 { Some('p') }
                else if (self.white_knights & bit) != 0 { Some('N') }
                else if (self.black_knights & bit) != 0 { Some('n') }
                else if (self.white_bishops & bit) != 0 { Some('B') }
                else if (self.black_bishops & bit) != 0 { Some('b') }
                else if (self.white_rooks & bit) != 0 { Some('R') }
                else if (self.black_rooks & bit) != 0 { Some('r') }
                else if (self.white_queens & bit) != 0 { Some('Q') }
                else if (self.black_queens & bit) != 0 { Some('q') }
                else if (self.white_king & bit) != 0 { Some('K') }
                else if (self.black_king & bit) != 0 { Some('k') }
                else { None };

                if let Some(c) = piece_char {
                    if empty_count > 0 {
                        fen.push((b'0' + empty_count) as char);
                        empty_count = 0;
                    }
                    fen.push(c);
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push((b'0' + empty_count) as char);
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');

        // 2. Active color
        match self.active_color {
            Color::White => fen.push('w'),
            Color::Black => fen.push('b'),
        }

        fen.push(' ');

        // 3. Castling rights
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if (self.castling_rights & 0b0001) != 0 { fen.push('K'); }
            if (self.castling_rights & 0b0010) != 0 { fen.push('Q'); }
            if (self.castling_rights & 0b0100) != 0 { fen.push('k'); }
            if (self.castling_rights & 0b1000) != 0 { fen.push('q'); }
        }

        fen.push(' ');

        // 4. En passant target square
        if self.en_passant_target == 0 {
            fen.push('-');
        } else {
            // Find the index of the set bit (0-63)
            let idx = self.en_passant_target.trailing_zeros();
            let file = (idx % 8) as u8;
            let rank = (idx / 8) as u8;
            fen.push((b'a' + file) as char);
            fen.push((b'1' + rank) as char);
        }

        fen.push(' ');

        // 5. Half-move clock
        fen.push_str(&self.half_move_clock.to_string());

        fen.push(' ');

        // 6. Full-move number
        fen.push_str(&self.full_move_number.to_string());

        fen
    }

    pub fn is_legal(&self) -> bool {
        // 1. One king per side
        if self.white_king.count_ones() != 1 || self.black_king.count_ones() != 1 {
            return false;
        }

        // 2. No pawns on ranks 1 or 8
        let rank_1_8_mask: u64 = 0xFF00_0000_0000_00FF;
        if (self.white_pawns & rank_1_8_mask) != 0 || (self.black_pawns & rank_1_8_mask) != 0 {
            return false;
        }

        // 3. Max 8 pawns per side
        if self.white_pawns.count_ones() > 8 || self.black_pawns.count_ones() > 8 {
            return false;
        }

        // 4. The Ghost Check: The opponent cannot be in check!
        let (opponent_king, active_color) = if self.active_color == Color::White {
            (self.black_king, Color::White) // If White's turn, Black King cannot be attacked by White
        } else {
            (self.white_king, Color::Black) // If Black's turn, White King cannot be attacked by Black
        };

        #[allow(clippy::cast_possible_truncation)]
        let opponent_king_square = opponent_king.trailing_zeros() as u8;

        if self.is_square_attacked(opponent_king_square, active_color) {
            return false;
        }

        // 5. En Passant Geometry
        if self.en_passant_target != 0 {
            let ep_square = self.en_passant_target.trailing_zeros();
            // Valid EP targets are strictly on Rank 3 (squares 16-23) or Rank 6 (squares 40-47)
            if !(16..=23).contains(&ep_square) && !(40..=47).contains(&ep_square) {
                return false;
            }
        }

        true
    }

    pub fn is_square_attacked(&self, square: u8, attacker: Color) -> bool {
        let square_bitboard = 1u64 << square;
        
        let (
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            king,
        ) = if attacker == Color::White {
            (
                self.white_pawns,
                self.white_knights,
                self.white_bishops,
                self.white_rooks,
                self.white_queens,
                self.white_king,
            )
        } else {
            (
                self.black_pawns,
                self.black_knights,
                self.black_bishops,
                self.black_rooks,
                self.black_queens,
                self.black_king,
            )
        };

        let pawn_attacks = if attacker == Color::White {
            ((square_bitboard & !A_FILE) >> 9) | ((square_bitboard & !H_FILE) >> 7)
        } else {
            ((square_bitboard & !A_FILE) << 7) | ((square_bitboard & !H_FILE) << 9)
        };

        if (pawn_attacks & pawns) != 0 {
            return true;
        }

        if (get_knight_attacks(square) & knights) != 0 {
            return true;
        }

        let diagonal_attackers = bishops | queens;
        if diagonal_attackers != 0 {
            let diagonal_attacks = get_bishop_attacks(square, self.all_occupancy);
            if (diagonal_attacks & diagonal_attackers) != 0 {
                return true;
            }
        }

        let orthogonal_attackers = rooks | queens;
        if orthogonal_attackers != 0 {
            let orthogonal_attacks = get_rook_attacks(square, self.all_occupancy);
            if (orthogonal_attacks & orthogonal_attackers) != 0 {
                return true;
            }
        }

        if (get_king_attacks(square) & king) != 0 {
            return true;
        }

        false
    }

    pub fn piece_type_at(&self, square: u8, color: Color) -> Option<PieceType> {
        let square_bitboard = 1u64 << square;

        if color == Color::White {
            if (self.white_pawns & square_bitboard) != 0 { return Some(PieceType::Pawn); }
            if (self.white_knights & square_bitboard) != 0 { return Some(PieceType::Knight); }
            if (self.white_bishops & square_bitboard) != 0 { return Some(PieceType::Bishop); }
            if (self.white_rooks & square_bitboard) != 0 { return Some(PieceType::Rook); }
            if (self.white_queens & square_bitboard) != 0 { return Some(PieceType::Queen); }
            if (self.white_king & square_bitboard) != 0 { return Some(PieceType::King); }
        } else {
            if (self.black_pawns & square_bitboard) != 0 { return Some(PieceType::Pawn); }
            if (self.black_knights & square_bitboard) != 0 { return Some(PieceType::Knight); }
            if (self.black_bishops & square_bitboard) != 0 { return Some(PieceType::Bishop); }
            if (self.black_rooks & square_bitboard) != 0 { return Some(PieceType::Rook); }
            if (self.black_queens & square_bitboard) != 0 { return Some(PieceType::Queen); }
            if (self.black_king & square_bitboard) != 0 { return Some(PieceType::King); }
        }

        None
    }

   pub fn make_move(&mut self, mv: Move) {
        let source_bitboard = 1u64 << mv.source;
        let target_bitboard = 1u64 << mv.target;
        let move_mask = source_bitboard | target_bitboard;

        let moving_piece = self.piece_type_at(mv.source, self.active_color)
            .expect("Attempted to move non-existent piece");

        let enemy_color = if self.active_color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        let captured_piece = self.piece_type_at(mv.target, enemy_color);

        if moving_piece == PieceType::Pawn
            && captured_piece.is_none()
            && (mv.source % 8) != (mv.target % 8) {
            let ep_capture_square = if self.active_color == Color::White {
                mv.target - 8
            } else {
                mv.target + 8
            };

            let ep_capture_bitboard = 1u64 << ep_capture_square;

            if self.active_color == Color::White {
                self.black_pawns ^= ep_capture_bitboard;
            } else {
                self.white_pawns ^= ep_capture_bitboard;
            }
        }

        #[inline]
        fn get_bitboard_mut(board: &mut Board, color: Color, piece: PieceType) -> &mut u64 {
            match (color, piece) {
                (Color::White, PieceType::Pawn) => &mut board.white_pawns,
                (Color::White, PieceType::Knight) => &mut board.white_knights,
                (Color::White, PieceType::Bishop) => &mut board.white_bishops,
                (Color::White, PieceType::Rook) => &mut board.white_rooks,
                (Color::White, PieceType::Queen) => &mut board.white_queens,
                (Color::White, PieceType::King) => &mut board.white_king,
                (Color::Black, PieceType::Pawn) => &mut board.black_pawns,
                (Color::Black, PieceType::Knight) => &mut board.black_knights,
                (Color::Black, PieceType::Bishop) => &mut board.black_bishops,
                (Color::Black, PieceType::Rook) => &mut board.black_rooks,
                (Color::Black, PieceType::Queen) => &mut board.black_queens,
                (Color::Black, PieceType::King) => &mut board.black_king,
            }
        }

        if let Some(piece) = captured_piece {
            *get_bitboard_mut(self, enemy_color, piece) ^= target_bitboard;
        }

        *get_bitboard_mut(self, self.active_color, moving_piece) ^= move_mask;

        if let Some(promotion_piece) = mv.promotion {
            if self.active_color == Color::White {
                self.white_pawns ^= target_bitboard;
            } else {
                self.black_pawns ^= target_bitboard;
            }

            *get_bitboard_mut(self, self.active_color, promotion_piece) |= target_bitboard;
        }

        self.castling_rights &= CASTLING_RIGHTS_UPDATE[mv.source as usize]
            & CASTLING_RIGHTS_UPDATE[mv.target as usize];

        self.en_passant_target = 0;

        if moving_piece == PieceType::Pawn {
            let distance = (mv.target as i8 - mv.source as i8).abs();

            if distance == 16 {
                let ep_target = if self.active_color == Color::White {
                    mv.source + 8
                } else {
                    mv.source - 8
                };

                self.en_passant_target = 1u64 << ep_target;
            }
        }

        if moving_piece == PieceType::Pawn || captured_piece.is_some() {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        if self.active_color == Color::Black {
            self.full_move_number += 1;
        }

        if moving_piece == PieceType::King {
            let distance = (mv.target as i8 - mv.source as i8).abs();
            if distance == 2 {
                match mv.target {
                    6 => self.white_rooks ^= (1u64 << 7) | (1u64 << 5),
                    2 => self.white_rooks ^= (1u64 << 0) | (1u64 << 3),
                    62 => self.black_rooks ^= (1u64 << 63) | (1u64 << 61),
                    58 => self.black_rooks ^= (1u64 << 56) | (1u64 << 59),
                    _ => unreachable!("Invalid castling target square"),
                }
            }
        }

        self.white_occupancy = self.white_pawns | self.white_knights | self.white_bishops |
            self.white_rooks | self.white_queens | self.white_king;
        self.black_occupancy = self.black_pawns | self.black_knights | self.black_bishops |
            self.black_rooks | self.black_queens | self.black_king;
        self.all_occupancy = self.white_occupancy | self.black_occupancy;

        self.active_color = enemy_color;
    }

}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
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
                        return Err(FenError::InvalidFileCountInRank(rank));
                    }

                    continue;
                }

                if byte.is_ascii_alphabetic() {
                    if file_index >= 8 {
                        let rank = rank_index + 1;
                        return Err(FenError::InvalidFileCountInRank(rank));
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
                return Err(FenError::InvalidFileCountInRank(rank));
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
            board.en_passant_target = 1u64 << (rank * 8 + file);
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
