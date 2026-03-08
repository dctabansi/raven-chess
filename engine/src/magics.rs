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
