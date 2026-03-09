pub use crate::magics::{get_bishop_attacks, get_queen_attacks, get_rook_attacks};

const A_FILE: u64 = 0x0101_0101_0101_0101;
const AB_FILE: u64 = 0x0303_0303_0303_0303;
const H_FILE: u64 = 0x8080_8080_8080_8080;
const GH_FILE: u64 = 0xC0C0_C0C0_C0C0_C0C0;

const KNIGHT_ATTACKS: [u64; 64] = generate_all_knight_attacks();
const KING_ATTACKS: [u64; 64] = generate_all_king_attacks();

#[inline(always)]
#[must_use]
pub fn get_knight_attacks(square: u8) -> u64 {
    KNIGHT_ATTACKS[square as usize]
}

#[inline(always)]
#[must_use]
pub fn get_king_attacks(square: u8) -> u64 {
    KING_ATTACKS[square as usize]
}

const fn mask_knight_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let bitboard: u64 = 1u64 << square;

    attacks |= (bitboard & !A_FILE) << 15;  // North-North-West
    attacks |= (bitboard & !H_FILE) << 17;  // North-North-East
    attacks |= (bitboard & !GH_FILE) << 10; // East-North-East
    attacks |= (bitboard & !GH_FILE) >> 6;  // East-South-East
    attacks |= (bitboard & !H_FILE) >> 15;  // South-South-East
    attacks |= (bitboard & !A_FILE) >> 17;  // South-South-West
    attacks |= (bitboard & !AB_FILE) >> 10; // West-South-West
    attacks |= (bitboard & !AB_FILE) << 6;  // West-North-West

    attacks
}

const fn generate_all_knight_attacks() -> [u64; 64] {
    let mut table = [0; 64];
    let mut square = 0;

    while square < 64 {
        table[square] = mask_knight_attacks(square as u8);
        square += 1;
    }

    table
}

const fn mask_king_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let bitboard: u64 = 1u64 << square;

    attacks |= (bitboard & !A_FILE) << 7;  // North-West
    attacks |= bitboard << 8;              // North
    attacks |= (bitboard & !H_FILE) << 9;  // North-East
    attacks |= (bitboard & !H_FILE) << 1;  // East
    attacks |= (bitboard & !H_FILE) >> 7;  // South-East
    attacks |= bitboard >> 8;              // South
    attacks |= (bitboard & !A_FILE) >> 9;  // South-West
    attacks |= (bitboard & !A_FILE) >> 1;  // West

    attacks
}

const fn generate_all_king_attacks() -> [u64; 64] {
    let mut table = [0; 64];
    let mut square = 0;

    while square < 64 {
        table[square] = mask_king_attacks(square as u8);
        square += 1;
    }

    table
}
