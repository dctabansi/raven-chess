use engine::board::Board;
use engine::init;
use engine::movegen::MoveGenerator;
use std::time::Instant;

//noinspection SpellCheckingInspection
fn main() {
    init();
    println!("RavenChess Engine Magics Initialized.\n");

    // Standard starting position
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen).expect("Failed to parse FEN");

    // Depth 5 from the start position should yield EXACTLY 4,865,609 nodes.
    perft_divide(&board, 5);
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
        let mv = movegen.moves.moves[i];
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
        let mv = movegen.moves.moves[i];
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
