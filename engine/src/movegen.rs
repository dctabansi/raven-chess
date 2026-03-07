use crate::attacks::{KING_ATTACKS, KNIGHT_ATTACKS};
use crate::board::Board;
use crate::types::{BitboardIterator, Color, Move, MoveList, PieceType};

const A_FILE: u64 = 0x0101_0101_0101_0101;
const H_FILE: u64 = 0x8080_8080_8080_8080;

const RANK_4: u64 = 0x0000_0000_FF00_0000;
const RANK_5: u64 = 0x0000_00FF_0000_0000;

pub struct MoveGenerator<'a> {
    board: &'a Board,
    pub moves: MoveList,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            moves: MoveList::new(),
        }
    }

    pub fn generate_moves(&mut self) {
    }

    fn generate_pseudo_legal_moves(&mut self) {
        self.moves.clear();

        if self.board.active_color == Color::White {
            self.generate_white_pawn_moves();
        } else {
            self.generate_black_pawn_moves();
        }

        self.generate_knight_moves();
        self.generate_bishop_moves();
        self.generate_rook_moves();
        self.generate_queen_moves();
        self.generate_king_moves();
    }

    fn generate_white_pawn_moves(&mut self) {
        #[inline(always)]
        fn add_move(moves: &mut MoveList, source: u8, target: u8) {
            if target >= 56 {
                moves.push(Move { source, target, promotion: Some(PieceType::Queen) });
                moves.push(Move { source, target, promotion: Some(PieceType::Rook) });
                moves.push(Move { source, target, promotion: Some(PieceType::Bishop) });
                moves.push(Move { source, target, promotion: Some(PieceType::Knight) });
            } else {
                moves.push(Move { source, target, promotion: None });
            }
        }

        let single_pushes = (self.board.white_pawns << 8) & !self.board.all_occupancy;
        for target in BitboardIterator(single_pushes) {
            add_move(&mut self.moves, target - 8, target);
        }

        let double_pushes = (single_pushes << 8) & !self.board.all_occupancy & RANK_4;
        for target in BitboardIterator(double_pushes) {
            self.moves.push(Move { source: target - 16, target, promotion: None });
        }

        let nw_attacks = (self.board.white_pawns & !A_FILE) << 7 & self.board.black_occupancy;
        for target in BitboardIterator(nw_attacks) {
            add_move(&mut self.moves, target - 7, target);
        }

        let ne_attacks = (self.board.white_pawns & !H_FILE) << 9 & self.board.black_occupancy;
        for target in BitboardIterator(ne_attacks) {
            add_move(&mut self.moves, target - 9, target);
        }

        let ep_nw = (self.board.white_pawns & !A_FILE) << 7 & self.board.en_passant_target;
        for target in BitboardIterator(ep_nw) {
            self.moves.push(Move { source: target - 7, target, promotion: None });
        }

        let ep_ne = (self.board.white_pawns & !H_FILE) << 9 & self.board.en_passant_target;
        for target in BitboardIterator(ep_ne) {
            self.moves.push(Move { source: target - 9, target, promotion: None });
        }
    }

    fn generate_black_pawn_moves(&mut self) {
        #[inline(always)]
        fn add_move(moves: &mut MoveList, source: u8, target: u8) {
            if target <= 7 {
                moves.push(Move { source, target, promotion: Some(PieceType::Queen) });
                moves.push(Move { source, target, promotion: Some(PieceType::Rook) });
                moves.push(Move { source, target, promotion: Some(PieceType::Bishop) });
                moves.push(Move { source, target, promotion: Some(PieceType::Knight) });

            } else {
                moves.push(Move { source, target, promotion: None });
            }
        }

        let single_pushes = (self.board.black_pawns >> 8) & !self.board.all_occupancy;
        for target in BitboardIterator(single_pushes) {
            add_move(&mut self.moves, target + 8, target);
        }

        let double_pushes = (single_pushes >> 8) & !self.board.all_occupancy & RANK_5;
        for target in BitboardIterator(double_pushes) {
            self.moves.push(Move { source: target + 16, target, promotion: None });
        }

        let sw_attacks = (self.board.black_pawns & !A_FILE) >> 9 & self.board.white_occupancy;
        for target in BitboardIterator(sw_attacks) {
            add_move(&mut self.moves, target + 9, target);
        }

        let se_attacks = (self.board.black_pawns & !H_FILE) >> 7 & self.board.white_occupancy;
        for target in BitboardIterator(se_attacks) {
            add_move(&mut self.moves, target + 7, target);
        }

        let ep_sw = (self.board.black_pawns & !A_FILE) >> 9 & self.board.en_passant_target;
        for target in BitboardIterator(ep_sw) {
            self.moves.push(Move { source: target + 9, target, promotion: None });
        }

        let ep_se = (self.board.black_pawns & !H_FILE) >> 7 & self.board.en_passant_target;
        for target in BitboardIterator(ep_se) {
            self.moves.push(Move { source: target + 7, target, promotion: None });
        }
    }

    fn generate_knight_moves(&mut self) {
        let (knights, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_knights, self.board.white_occupancy)
        } else {
            (self.board.black_knights, self.board.black_occupancy)
        };

        self.generate_leaper_moves(knights, occupancy, &KNIGHT_ATTACKS);
    }

    fn generate_bishop_moves(&mut self) {}

    fn generate_rook_moves(&mut self) {}

    fn generate_queen_moves(&mut self) {}

    fn generate_king_moves(&mut self) {
        let (king, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_king, self.board.white_occupancy)
        } else {
            (self.board.black_king, self.board.black_occupancy)
        };

        self.generate_leaper_moves(king, occupancy, &KING_ATTACKS);

        // TODO: Handle castling
    }

    #[inline(always)]
    fn generate_leaper_moves(&mut self, pieces: u64, occupancy: u64, attacks_table: &[u64; 64]) {
        for source in BitboardIterator(pieces) {
            let attacks = attacks_table[source as usize] & !occupancy;

            for target in BitboardIterator(attacks) {
                self.moves.push(Move { source, target, promotion: None });
            }
        }
    }

}
