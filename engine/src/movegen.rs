use crate::attacks::*;
use crate::board::Board;
use crate::constants::{A_FILE, BK_CASTLING, BK_EMPTY_MASK, BQ_CASTLING, BQ_EMPTY_MASK, H_FILE, RANK_4, RANK_5, WK_CASTLING, WK_EMPTY_MASK, WQ_CASTLING, WQ_EMPTY_MASK};
use crate::types::{BitboardIterator, Color, Move, MoveList, PieceType};

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
        self.generate_pseudo_legal_moves();

        let pseudo_moves = self.moves.clone();
        self.moves.clear();

        let (active_king, enemy_color) = if self.board.active_color == Color::White {
            (self.board.white_king, Color::Black)
        } else {
            (self.board.black_king, Color::White)
        };

        for i in 0..pseudo_moves.count {
            let mv = pseudo_moves.moves[i];
            let mut test_board = self.board.clone();

            test_board.make_move(mv);

            #[allow(clippy::cast_possible_truncation)]
            let current_king_square = if mv.source == active_king.trailing_zeros() as u8 {
                mv.target
            } else {
                active_king.trailing_zeros() as u8
            };

            if !test_board.is_square_attacked(current_king_square, enemy_color) {
                self.moves.push(mv);
            }
        }

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
        self.generate_castling_moves();
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

        let se_attacks = (self.board.black_pawns & !H_FILE) >> 7 & self.board.white_occupancy;
        for target in BitboardIterator(se_attacks) {
            add_move(&mut self.moves, target + 7, target);
        }
        
        let sw_attacks = (self.board.black_pawns & !A_FILE) >> 9 & self.board.white_occupancy;
        for target in BitboardIterator(sw_attacks) {
            add_move(&mut self.moves, target + 9, target);
        }
        
        let ep_se = (self.board.black_pawns & !H_FILE) >> 7 & self.board.en_passant_target;
        for target in BitboardIterator(ep_se) {
            self.moves.push(Move { source: target + 7, target, promotion: None });
        }

        let ep_sw = (self.board.black_pawns & !A_FILE) >> 9 & self.board.en_passant_target;
        for target in BitboardIterator(ep_sw) {
            self.moves.push(Move { source: target + 9, target, promotion: None });
        }
    }

    fn generate_knight_moves(&mut self) {
        let (knights, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_knights, self.board.white_occupancy)
        } else {
            (self.board.black_knights, self.board.black_occupancy)
        };

        self.generate_leaper_moves(knights, occupancy, get_knight_attacks);
    }

    fn generate_bishop_moves(&mut self) {
        let (bishops, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_bishops, self.board.white_occupancy)
        } else {
            (self.board.black_bishops, self.board.black_occupancy)
        };

        self.generate_slider_moves(bishops, occupancy, get_bishop_attacks);
    }

    fn generate_rook_moves(&mut self) {
        let (rooks, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_rooks, self.board.white_occupancy)
        } else {
            (self.board.black_rooks, self.board.black_occupancy)
        };

        self.generate_slider_moves(rooks, occupancy, get_rook_attacks);
    }

    fn generate_queen_moves(&mut self) {
        let (queens, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_queens, self.board.white_occupancy)
        } else {
            (self.board.black_queens, self.board.black_occupancy)
        };

        self.generate_slider_moves(queens, occupancy, get_queen_attacks);
    }

    fn generate_king_moves(&mut self) {
        let (king, occupancy) = if self.board.active_color == Color::White {
            (self.board.white_king, self.board.white_occupancy)
        } else {
            (self.board.black_king, self.board.black_occupancy)
        };

        self.generate_leaper_moves(king, occupancy, get_king_attacks);
    }

    fn generate_castling_moves(&mut self) {
        match self.board.active_color {
            Color::White => {
                let enemy_color = Color::Black;

                if (self.board.castling_rights & WK_CASTLING) != 0
                    && (self.board.all_occupancy & WK_EMPTY_MASK) == 0
                    && !self.board.is_square_attacked(4, enemy_color)
                    && !self.board.is_square_attacked(5, enemy_color)
                    && !self.board.is_square_attacked(6, enemy_color)
                {
                    self.moves.push(Move { source: 4, target: 6, promotion: None });
                }

                if (self.board.castling_rights & WQ_CASTLING) != 0
                    && (self.board.all_occupancy & WQ_EMPTY_MASK) == 0
                    && !self.board.is_square_attacked(4, enemy_color)
                    && !self.board.is_square_attacked(3, enemy_color)
                    && !self.board.is_square_attacked(2, enemy_color)
                {
                    self.moves.push(Move { source: 4, target: 2, promotion: None });
                }
            }
            Color::Black => {
                let enemy_color = Color::White;

                if (self.board.castling_rights & BK_CASTLING) != 0
                    && (self.board.all_occupancy & BK_EMPTY_MASK) == 0
                    && !self.board.is_square_attacked(60, enemy_color)
                    && !self.board.is_square_attacked(61, enemy_color)
                    && !self.board.is_square_attacked(62, enemy_color)
                {
                    self.moves.push(Move { source: 60, target: 62, promotion: None });
                }

                if (self.board.castling_rights & BQ_CASTLING) != 0
                    && (self.board.all_occupancy & BQ_EMPTY_MASK) == 0
                    && !self.board.is_square_attacked(60, enemy_color)
                    && !self.board.is_square_attacked(59, enemy_color)
                    && !self.board.is_square_attacked(58, enemy_color)
                {
                    self.moves.push(Move { source: 60, target: 58, promotion: None });
                }
            }
        }
    }

    #[inline(always)]
    fn generate_leaper_moves<F>(&mut self, pieces: u64, occupancy: u64, get_attacks: F)
    where F: Fn(u8) -> u64{
        for source in BitboardIterator(pieces) {
            let attacks = get_attacks(source) & !occupancy;

            for target in BitboardIterator(attacks) {
                self.moves.push(Move { source, target, promotion: None });
            }
        }
    }

    #[inline(always)]
    fn generate_slider_moves<F>(&mut self, pieces: u64, friendly_occupancy: u64, get_attacks: F)
    where F: Fn(u8, u64) -> u64 {
        for source in BitboardIterator(pieces) {
            let raw_attacks = get_attacks(source, self.board.all_occupancy);
            let valid_attacks = raw_attacks & !friendly_occupancy;

            for target in BitboardIterator(valid_attacks) {
                self.moves.push(Move { source, target, promotion: None });
            }
        }
    }

}
