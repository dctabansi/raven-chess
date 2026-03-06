use crate::board::Board;
use crate::types::{Color, Move, MoveList, PieceType};

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
        }
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

        let mut single_pushes = (self.board.white_pawns << 8) & !self.board.all_occupancy;

        let valid_single_pushes = single_pushes;

        while single_pushes != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = single_pushes.trailing_zeros() as u8;
            let source_square = target_square - 8;

            add_move(&mut self.moves, source_square, target_square);

            single_pushes &= single_pushes - 1;
        }

        let mut double_pushes = (valid_single_pushes << 8) & !self.board.all_occupancy & RANK_4;

        while double_pushes != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = double_pushes.trailing_zeros() as u8;
            let source_square = target_square - 16;

            self.moves.push(Move {
                source: source_square,
                target: target_square,
                promotion: None,
            });

            double_pushes &= double_pushes - 1;
        }

        let mut nw_attacks = (self.board.white_pawns & !A_FILE) << 7 & self.board.black_occupancy;

        while nw_attacks != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = nw_attacks.trailing_zeros() as u8;
            let source_square = target_square - 7;

            add_move(&mut self.moves, source_square, target_square);

            nw_attacks &= nw_attacks - 1;
        }

        let mut ne_attacks = (self.board.white_pawns & !H_FILE) << 9 & self.board.black_occupancy;

        while ne_attacks != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = ne_attacks.trailing_zeros() as u8;
            let source_square = target_square - 9;

            add_move(&mut self.moves, source_square, target_square);

            ne_attacks &= ne_attacks - 1;
        }

        let mut ep_nw = (self.board.white_pawns & !A_FILE) << 7 & self.board.en_passant_target;

        while ep_nw != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target = ep_nw.trailing_zeros() as u8;
            let source = target - 7;

            self.moves.push(Move { source, target, promotion: None });

            ep_nw &= ep_nw - 1;
        }

        let mut ep_ne = (self.board.white_pawns & !H_FILE) << 9 & self.board.en_passant_target;

        while ep_ne != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target = ep_ne.trailing_zeros() as u8;
            let source = target - 9;

            self.moves.push(Move { source, target, promotion: None });

            ep_ne &= ep_ne - 1;
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

        let mut single_pushes = (self.board.black_pawns >> 8) & !self.board.all_occupancy;

        let valid_single_pushes = single_pushes;

        while single_pushes != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = single_pushes.trailing_zeros() as u8;
            let source_square = target_square + 8;

            add_move(&mut self.moves, source_square, target_square);

            single_pushes &= single_pushes - 1;
        }

        let mut double_pushes = (valid_single_pushes >> 8) & !self.board.all_occupancy & RANK_5;

        while double_pushes != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = double_pushes.trailing_zeros() as u8;
            let source_square = target_square + 16;

            self.moves.push(Move {
                source: source_square,
                target: target_square,
                promotion: None,
            });

            double_pushes &= double_pushes - 1;
        }

        let mut sw_attacks = (self.board.black_pawns & !A_FILE) >> 9 & self.board.white_occupancy;

        while sw_attacks != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = sw_attacks.trailing_zeros() as u8;
            let source_square = target_square + 9;

            add_move(&mut self.moves, source_square, target_square);

            sw_attacks &= sw_attacks - 1;
        }

        let mut se_attacks = (self.board.black_pawns & !H_FILE) >> 7 & self.board.white_occupancy;

        while se_attacks != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target_square = se_attacks.trailing_zeros() as u8;
            let source_square = target_square + 7;

            add_move(&mut self.moves, source_square, target_square);

            se_attacks &= se_attacks - 1;
        }

        let mut ep_sw = (self.board.black_pawns & !A_FILE) >> 9 & self.board.en_passant_target;

        while ep_sw != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target = ep_sw.trailing_zeros() as u8;
            let source = target + 9;

            self.moves.push(Move { source, target, promotion: None });

            ep_sw &= ep_sw - 1;
        }

        let mut ep_se = (self.board.black_pawns & !H_FILE) >> 7 & self.board.en_passant_target;

        while ep_se != 0 {
            #[allow(clippy::cast_possible_truncation)]
            let target = ep_se.trailing_zeros() as u8;
            let source = target + 7;

            self.moves.push(Move { source, target, promotion: None });

            ep_se &= ep_se - 1;
        }
    }

}
