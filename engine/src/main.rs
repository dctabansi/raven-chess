use engine::board::Board;
use engine::init;
use engine::movegen::MoveGenerator;
use engine::search::search_position;
use std::time::Instant;

//noinspection SpellCheckingInspection
fn main() {
    init();
    println!("RavenChess Engine Magics Initialized.\n");

    // Standard starting position
    let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board1 = Board::from_fen(fen1).expect("Failed to parse FEN");

    let fen2 = "r1bq2r1/b4pk1/p1pp1p2/1p2pP2/1P2P1PB/3P4/1PPQ2P1/R3K2R w KQ - 1 20";
    let board2 = Board::from_fen(fen2).expect("Failed to parse FEN");

    // Depth 5 from the start position should yield EXACTLY 4,865,609 nodes.
    // perft_divide(&board, 5);

    println!("Search on FEN: {fen1}");
    run_search(&board1, 5);

    println!("Search on FEN: {fen2}");
    run_search(&board2, 4);
}

// The recursive tree-walking function
fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();

    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];
        let mut next_board = board.clone();
        next_board.make_move(mv);

        nodes += perft(&next_board, depth - 1);
    }

    nodes
}

pub fn perft_divide(board: &Board, depth: u8) -> u64 {
    println!("--- Perft Divide Depth {} ---", depth);
    let start_time = Instant::now();

    let mut total_nodes = 0;
    let mut movegen = MoveGenerator::new(board);
    movegen.generate_moves();

    for i in 0..movegen.moves.count {
        let mv = movegen.moves[i];
        let mut next_board = board.clone();
        next_board.make_move(mv);

        let nodes = perft(&next_board, depth - 1);
        total_nodes += nodes;

        // Convert squares to algebraic notation for readability
        let source_file = (mv.source % 8) + b'a';
        let source_rank = (mv.source / 8) + b'1';
        let target_file = (mv.target % 8) + b'a';
        let target_rank = (mv.target / 8) + b'1';

        println!(
            "{}{}{}{}: {}",
            source_file as char, source_rank as char, target_file as char, target_rank as char, nodes
        );
    }

    let duration = start_time.elapsed();
    println!("\nTotal Nodes: {}", total_nodes);
    println!("Time taken: {:?}", duration);

    // Calculate Nodes Per Second (NPS)
    let nps = (total_nodes as f64 / duration.as_secs_f64()) as u64;
    println!("NPS: {}\n", nps);

    total_nodes
}

pub fn run_search(board: &Board, depth: u8) {
    println!("Searching depth {depth}...");
    let start_time = Instant::now();

    let (best_move, score) = search_position(board, depth);
    let duration = start_time.elapsed();

    if let Some(mv) = best_move {
        let source_file = (mv.source % 8) + b'a';
        let source_rank = (mv.source / 8) + b'1';
        let target_file = (mv.target % 8) + b'a';
        let target_rank = (mv.target / 8) + b'1';

        let promotion_char = match mv.promotion {
            Some(engine::types::PieceType::Queen) => "q",
            Some(engine::types::PieceType::Rook) => "r",
            Some(engine::types::PieceType::Bishop) => "b",
            Some(engine::types::PieceType::Knight) => "n",
            _ => "",
        };

        println!("Best move: {}{}{}{}{}",
                 source_file as char,
                 source_rank as char,
                 target_file as char,
                 target_rank as char,
                 promotion_char
        );

        if score > 48000 {
            println!("Evaluation: Mate found");
        } else if score < -48000 {
            println!("Evaluation: Getting mated");
        } else {
            println!("Evaluation: {:.2} pawns ({} cp)", score as f32 / 100.0, score);
        }

        println!("Time taken: {:?}", duration);
        println!("----------------------------------------");
    } else {
        println!("No legal moves available (Checkmate or Stalemate).");
    }
}
