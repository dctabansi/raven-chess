use std::sync::Once;
use crate::types::{Color, PieceType};

pub static mut PIECE_KEYS: [[u64; 64]; 12] = [[0; 64]; 12];
pub static mut EN_PASSANT_KEYS: [u64; 8] = [0; 8];
pub static mut CASTLE_KEYS: [u64; 16] = [0; 16];
pub static mut SIDE_KEY: u64 = 0;

static INIT: Once = Once::new();

pub fn init_zobrist() {
    INIT.call_once(|| {
        let mut prng = Prng::new(1010);
        
        let mut piece_keys = [[0; 64]; 12];
        for piece_row in piece_keys.iter_mut() {
            for square in piece_row.iter_mut() {
                *square = prng.random_sparse_u64();
            }
        }

        let mut en_passant_keys = [0; 8];
        for en_passant_key in en_passant_keys.iter_mut() {
            *en_passant_key = prng.random_sparse_u64();
        }

        let mut castle_keys = [0; 16];
        for castle_key in castle_keys.iter_mut() {
            *castle_key = prng.random_sparse_u64();
        }

        let side_key = prng.random_sparse_u64();

        unsafe {
            PIECE_KEYS = piece_keys;
            EN_PASSANT_KEYS = en_passant_keys;
            CASTLE_KEYS = castle_keys;
            SIDE_KEY = side_key;
        }
    })
}

#[inline(always)]
pub fn get_piece_index(color: Color, piece_type: PieceType) -> usize {
    let color_offset = if color == Color::White { 0 } else { 6 };
    
    let piece_offset = match piece_type {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Bishop => 2,
        PieceType::Rook => 3,
        PieceType::Queen => 4,
        PieceType::King => 5,
    };
    
    color_offset + piece_offset
}

pub struct Prng {
    state: u64,
}

impl Prng {
    pub fn new(seed: u64) -> Self {
        Prng { state: seed }
    }

    pub fn random_sparse_u64(&mut self) -> u64{
        self.rand64() & self.rand64() & self.rand64()
    }
    
    pub fn rand64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

