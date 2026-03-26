use crate::board::Board;
use crate::types::{BitboardIterator, Color, PieceType};

// Material values in centipawns
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

// Piece-Square Tables (PSTs)
#[rustfmt::skip]
const PAWN_PST: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0, // Rank 1
    50, 50,  50,  50,  50,  50,  50,  50, // Rank 2
    10, 10,  20,  30,  30,  20,  10,  10, // Rank 3
    5,   5,  10,  25,  25,  10,   5,   5, // Rank 4
    0,   0,   0,  20,  20,   0,   0,   0, // Rank 5
    5,  -5, -10,   0,   0, -10,  -5,   5, // Rank 6
    5,  10,  10, -20, -20,  10,  10,   5, // Rank 7
    0,   0,   0,   0,   0,   0,   0,   0, // Rank 8 (Promoted)
];

#[rustfmt::skip]
const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, // Rank 1
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50, // Rank 8
];

#[rustfmt::skip]
const BISHOP_PST: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, // Rank 1
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20, // Rank 8
];

// Flip the board vertically for Black's PST lookup
#[inline(always)]
fn mirror_square(square: u8) -> usize {
    (square ^ 56) as usize
}

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    // 1. Material Evaluation

    let white_material = (board.white_pawns.count_ones() as i32 * PAWN_VALUE) +
        (board.white_knights.count_ones() as i32 * KNIGHT_VALUE) +
        (board.white_bishops.count_ones() as i32 * BISHOP_VALUE) +
        (board.white_rooks.count_ones() as i32 * ROOK_VALUE) +
        (board.white_queens.count_ones() as i32 * QUEEN_VALUE);

    let black_material = (board.black_pawns.count_ones() as i32 * PAWN_VALUE) +
        (board.black_knights.count_ones() as i32 * KNIGHT_VALUE) +
        (board.black_bishops.count_ones() as i32 * BISHOP_VALUE) +
        (board.black_rooks.count_ones() as i32 * ROOK_VALUE) +
        (board.black_queens.count_ones() as i32 * QUEEN_VALUE);

    score += white_material - black_material;

    // 2. Positional Evaluation (PSTs)

    for square in BitboardIterator(board.white_pawns) {
        score += PAWN_PST[square as usize];
    }

    for square in BitboardIterator(board.black_pawns) {
        score -= PAWN_PST[mirror_square(square)];
    }

    for square in BitboardIterator(board.white_knights) {
        score += KNIGHT_PST[square as usize];
    }

    for square in BitboardIterator(board.black_knights) {
        score -= KNIGHT_PST[mirror_square(square)];
    }

    for square in BitboardIterator(board.white_bishops) {
        score += BISHOP_PST[square as usize];
    }

    for square in BitboardIterator(board.black_bishops) {
        score -= BISHOP_PST[mirror_square(square)];
    }

    // TODO: Add Rook, Queen, and King PSTs

    // 3. Negamax Adjustment

    if board.active_color == Color::White {
        score
    } else {
        -score
    }
}

pub fn get_piece_value(piece_type: PieceType) -> i32 {
    match piece_type {
        PieceType::Pawn => PAWN_VALUE,
        PieceType::Knight => KNIGHT_VALUE,
        PieceType::Bishop => BISHOP_VALUE,
        PieceType::Rook => ROOK_VALUE,
        PieceType::Queen => QUEEN_VALUE,
        PieceType::King => KING_VALUE,
    }
}
