use std::cmp::max;
use crate::board::Board;
use crate::eval::evaluate;
use crate::movegen::MoveGenerator;
use crate::types::{Color, Move};

const INFINITY: i32 = 50_000;
const MATE: i32 = 49_000;

pub fn search_position(board: &Board, depth: u8) -> (Option<Move>, i32) {
    let mut best_move = None;
    let mut max_score = -INFINITY;

    let mut alpha = -INFINITY;
    let beta = INFINITY;

    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();

    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];
        let mut next_board = board.clone();
        next_board.make_move(mv);

        // Negamax: score of next board state is inverted
        // swap and invert alpha and beta
        let score = -negamax(&next_board, depth - 1, -beta, -alpha);

        if score > max_score {
            max_score = score;
            best_move = Some(mv);
        }

        // update alpha bound
        alpha = max(alpha, max_score);
    }

    (best_move, max_score)
}

/// Negamax recursive search algorithm with Alpha-Beta pruning.
fn negamax(board: &Board, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    // 1. Base case: reached maximum search depth, evaluate the static board
    if depth == 0 {
        return evaluate(board);
    }

    // 2. Generate all legal moves
    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();

    // 3. Check for stalemate or checkmate
    if movegen.moves.count == 0 {
        let (king_bb, enemy_color) = if board.active_color == Color::White {
            (board.white_king, Color::Black)
        } else {
            (board.black_king, Color::White)
        };

        let king_sq = king_bb.trailing_zeros() as u8;

        return if board.is_square_attacked(king_sq, enemy_color) {
            // checkmate
            -MATE + (100 - depth as i32)
        } else {
            // stalemate
            0
        }
    }

    let mut max_score = -INFINITY;

    // 4. Recursive loop
    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];
        let mut next_board = board.clone();
        next_board.make_move(mv);

        let score = -negamax(&next_board, depth - 1, -beta, -alpha);

        // ALPHA-BETA PRUNING
        if score >= beta {
            return beta;
        }

        max_score = max(max_score, score);

        alpha = max(alpha, score);
    }

    max_score
}
