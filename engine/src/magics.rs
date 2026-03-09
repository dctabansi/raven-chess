use std::sync::Once;
use crate::magic_constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: usize,
}

macro_rules! ray_attacks {
    (
        $target_rank:expr, $target_file:expr,
        [ $( ($rank_step:expr, $file_step:expr, $rank:ident, $file:ident, $cond:expr) ),* $(,)? ]
    ) => {{
        let mut attacks: u64 = 0;
        $(
            let mut $rank = $target_rank + $rank_step;
            let mut $file = $target_file + $file_step;
            while $cond {
                attacks |= 1u64 << ($rank * 8 + $file);
                $rank += $rank_step;
                $file += $file_step;
            }
        )*
        attacks
    }};
    (
        $target_rank:expr, $target_file:expr, $block:expr,
        [ $( ($rank_step:expr, $file_step:expr, $rank:ident, $file:ident, $cond:expr) ),* $(,)? ]
    ) => {{
        let mut attacks: u64 = 0;
        $(
            let mut $rank = $target_rank + $rank_step;
            let mut $file = $target_file + $file_step;
            while $cond {
                let current_square = ($rank * 8 + $file) as u8;
                attacks |= 1u64 << current_square;
                if ($block & (1u64 << current_square)) != 0 {
                    break;
                }
                $rank += $rank_step;
                $file += $file_step;
            }
        )*
        attacks
    }};
}

// The global attack tables
pub static mut BISHOP_ATTACKS: [u64; 5248] = [0; 5248];
pub static mut ROOK_ATTACKS: [u64; 102400] = [0; 102400];

// The Magic lookup instructions for each square
pub static mut BISHOP_MAGICS: [Magic; 64] = [Magic { mask: 0, magic: 0, shift: 0, offset: 0 }; 64];
pub static mut ROOK_MAGICS: [Magic; 64] = [Magic { mask: 0, magic: 0, shift: 0, offset: 0 }; 64];

static INIT: Once = Once::new();

pub fn init_magics() {
    INIT.call_once(|| {
        let mut bishop_offset = 0;
        let mut rook_offset = 0;

        for square in 0..64 {
            let bishop_mask = mask_bishop_attacks(square);
            let bishop_bits = BISHOP_RELEVANT_BITS[square as usize];
            let bishop_permutations = 1 << bishop_bits;
            let bishop_magic = BISHOP_MAGICS_ARRAY[square as usize];

            unsafe {
                BISHOP_MAGICS[square as usize] = Magic {
                    mask: bishop_mask,
                    magic: bishop_magic,
                    shift: 64 - bishop_bits,
                    offset: bishop_offset,
                };
            }

            for index in 0..bishop_permutations {
                let occupancy = set_occupancy(index, bishop_mask);
                let attack = bishop_attacks_slow(square, occupancy);
                let magic_index = (occupancy.wrapping_mul(bishop_magic)) >> (64 - bishop_bits);

                unsafe {
                    BISHOP_ATTACKS[bishop_offset + magic_index as usize] = attack;
                }
            }

            bishop_offset += bishop_permutations;

            let rook_mask = mask_rook_attacks(square);
            let rook_bits = ROOK_RELEVANT_BITS[square as usize];
            let rook_permutations = 1 << rook_bits;
            let rook_magic = ROOK_MAGICS_ARRAY[square as usize];

            unsafe {
                ROOK_MAGICS[square as usize] = Magic {
                    mask: rook_mask,
                    magic: rook_magic,
                    shift: 64 - rook_bits,
                    offset: rook_offset,
                };
            }

            for index in 0..rook_permutations {
                let occupancy = set_occupancy(index, rook_mask);
                let attack = rook_attacks_slow(square, occupancy);
                let magic_index = (occupancy.wrapping_mul(rook_magic)) >> (64 - rook_bits);

                unsafe {
                    ROOK_ATTACKS[rook_offset + magic_index as usize] = attack;
                }
            }

            rook_offset += rook_permutations;
        }
    });
}

#[inline(always)]
fn get_magic_index(magic_entry: &Magic, occupancy: u64) -> usize {
    let blockers = occupancy & magic_entry.mask;
    ((blockers.wrapping_mul(magic_entry.magic)) >> magic_entry.shift) as usize
}

#[inline(always)]
pub fn get_bishop_attacks(square: u8, occupancy: u64) -> u64 {
    unsafe {
        let magic_entry = &BISHOP_MAGICS[square as usize];
        BISHOP_ATTACKS[magic_entry.offset + get_magic_index(magic_entry, occupancy)]
    }
}

#[inline(always)]
pub fn get_rook_attacks(square: u8, occupancy: u64) -> u64 {
    unsafe {
        let magic_entry = &ROOK_MAGICS[square as usize];
        ROOK_ATTACKS[magic_entry.offset + get_magic_index(magic_entry, occupancy)]
    }
}

#[inline(always)]
pub fn get_queen_attacks(square: u8, occupancy: u64) -> u64 {
    get_bishop_attacks(square, occupancy) | get_rook_attacks(square, occupancy)
}

pub const fn mask_bishop_attacks(square: u8) -> u64 {
    let target_rank = (square / 8) as i32;
    let target_file = (square % 8) as i32;

    ray_attacks!(
        target_rank, target_file,
        [
            (1, -1, rank, file, rank <= 6 && file >= 1),  // North-West
            (1, 1, rank, file, rank <= 6 && file <= 6),   // North-East
            (-1, 1, rank, file, rank >= 1 && file <= 6),  // South-East
            (-1, -1, rank, file, rank >= 1 && file >= 1), // South-West
        ]
    )
}

pub const fn mask_rook_attacks(square: u8) -> u64 {
    let target_rank = (square / 8) as i32;
    let target_file = (square % 8) as i32;

    ray_attacks!(
        target_rank, target_file,
        [
            (1, 0, rank, file, rank <= 6),  // North
            (0, 1, rank, file, file <= 6),  // East
            (-1, 0, rank, file, rank >= 1), // South
            (0, -1, rank, file, file >= 1), // West
        ]
    )
}

pub const fn set_occupancy(index: usize, mut mask: u64) -> u64 {
    let mut occupancy: u64 = 0;
    
    let mut i = 0;
    let count = mask.count_ones();

    while i < count {
        #[allow(clippy::cast_possible_truncation)]
        let square = mask.trailing_zeros() as u8;
        mask &= mask - 1;

        if (index & (1 << i)) != 0 {
            occupancy |= 1u64 << square;
        }

        i += 1;
    }

    occupancy
}

pub const fn bishop_attacks_slow(square: u8, block: u64) -> u64 {
    let target_rank = (square / 8) as i32;
    let target_file = (square % 8) as i32;

    ray_attacks!(
        target_rank, target_file, block,
        [
            (1, -1, rank, file, rank <= 7 && file >= 0),  // North-West
            (1, 1, rank, file, rank <= 7 && file <= 7),   // North-East
            (-1, 1, rank, file, rank >= 0 && file <= 7),  // South-East
            (-1, -1, rank, file, rank >= 0 && file >= 0), // South-West
        ]
    )
}

pub const fn rook_attacks_slow(square: u8, block: u64) -> u64 {
    let target_rank = (square / 8) as i32;
    let target_file = (square % 8) as i32;

    ray_attacks!(
        target_rank, target_file, block,
        [
            (1, 0, rank, file, rank <= 7),  // North
            (0, 1, rank, file, file <= 7),  // East
            (-1, 0, rank, file, rank >= 0), // South
            (0, -1, rank, file, file >= 0), // West
        ]
    )
}
