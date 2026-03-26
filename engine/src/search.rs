use std::cmp::max;
use crate::board::Board;
use crate::eval::{evaluate, get_piece_value};
use crate::movegen::MoveGenerator;
use crate::types::{Color, Move, PieceType};
use crate::tt::{TranspositionTable, TTFlag, TTEntry};

const INFINITY: i32 = 50_000;
const MATE: i32 = 49_000;

pub fn search_position(board: &Board, depth: u8, tt: &mut TranspositionTable) -> (Option<Move>, i32) {
    let mut best_move = None;
    let mut max_score = -INFINITY;

    let mut alpha = -INFINITY;
    let beta = INFINITY;

    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();

    // Sort moves so the highest scores are evaluated first (descending order)
    movegen.moves.moves[0..movegen.moves.count].sort_unstable_by_key(|mv| -score_move(board, mv));

    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];
        let mut next_board = board.clone();
        next_board.make_move(mv);

        // Negamax: score of next board state is inverted
        // swap and invert alpha and beta
        let score = -negamax(&next_board, depth - 1, -beta, -alpha, tt);

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
fn negamax(board: &Board, depth: u8, mut alpha: i32, beta: i32, tt: &mut TranspositionTable) -> i32 {
    // 1. Check Transposition Table
    let alpha_orig = alpha;
    if let Some(score) = tt.read(board.hash, depth, alpha, beta) {
        return score;
    }

    // 2. Base case: reached maximum search depth, evaluate the static board
    if depth == 0 {
        return qsearch(board, alpha, beta);
    }

    // 3. Generate all legal moves
    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();

    // 4. Check for stalemate or checkmate
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

    // Sort moves so the highest scores are evaluated first (descending order)
    movegen.moves.moves[0..movegen.moves.count].sort_unstable_by_key(|mv| -score_move(board, mv));

    let mut max_score = -INFINITY;
    let mut best_move: Option<Move> = None; // Track for TT

    // 5. Recursive loop
    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];
        let mut next_board = board.clone();
        next_board.make_move(mv);

        let score = -negamax(&next_board, depth - 1, -beta, -alpha, tt);

        if score > max_score {
            max_score = score;
            best_move = Some(mv); // Update the best move
        }

        alpha = max(alpha, max_score);

        // ALPHA-BETA PRUNING
        if alpha >= beta {
            break;
        }
    }

    // 6. Save evaluation to the Transposition Table
    let flag = if max_score <= alpha_orig {
        TTFlag::UpperBound // Failed low
    } else if max_score >= beta {
        TTFlag::LowerBound // Failed high
    } else {
        TTFlag::Exact
    };

    tt.write(board.hash, depth, max_score, flag, best_move);

    max_score
}

fn score_move(board: &Board, mv: &Move) -> i32 {
    let mut score = 0;
    let enemy_color = if board.active_color == Color::White { Color::Black } else { Color::White };

    // MV-LVA

    // 1. Captures
    if let Some(victim) = board.piece_type_at(mv.target, enemy_color) {
        let attacker = board.piece_type_at(mv.source, board.active_color)
            .expect("Attacker should exist");

        score += 10 * get_piece_value(victim) - get_piece_value(attacker);
    }

    // 2. Promotions
    if let Some(promo) = mv.promotion {
        score += get_piece_value(promo);
    }

    score
}

// Quiescence Search
fn qsearch(board: &Board, mut alpha: i32, beta: i32) -> i32 {
    // The "Stand Pat" score
    let static_eval = evaluate(board);

    if static_eval >= beta {
        return beta;
    }

    alpha = max(alpha, static_eval);

    // 2. Generate and sort moves
    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();
    movegen.moves.moves[0..movegen.moves.count].sort_unstable_by_key(|mv| -score_move(board, mv));

    let enemy_color = if board.active_color == Color::White { Color::Black } else { Color::White };

    // 3. Capture Loop
    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];

        let is_capture = board.piece_type_at(mv.target, enemy_color).is_some();
        let is_queen_promo = mv.promotion == Some(PieceType::Queen);

        // Filter: skip moves excluding captures or promotions
        if !is_capture && !is_queen_promo {
            continue;
        }

        let mut next_board = board.clone();
        next_board.make_move(mv);

        let score = -qsearch(&next_board, -beta, -alpha);

        if score >= beta {
            return beta;
        }

        alpha = max(alpha, score);
    }

    alpha
}
