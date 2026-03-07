const A_FILE: u64 = 0x0101_0101_0101_0101;
const AB_FILE: u64 = 0x0303_0303_0303_0303;
const H_FILE: u64 = 0x8080_8080_8080_8080;
const GH_FILE: u64 = 0xC0C0_C0C0_C0C0_C0C0;

pub const KNIGHT_ATTACKS: [u64; 64] = generate_all_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = generate_all_king_attacks();

const fn mask_knight_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let bitboard: u64 = 1u64 << square;

    // North jumps
    attacks |= (bitboard & !A_FILE) << 15;  // North-North-West
    attacks |= (bitboard & !H_FILE) << 17;  // North-North-East
    attacks |= (bitboard & !AB_FILE) << 6;  // West-North-West
    attacks |= (bitboard & !GH_FILE) << 10; // East-North-East

    // South jumps
    attacks |= (bitboard & !A_FILE) >> 17;  // South-South-West
    attacks |= (bitboard & !H_FILE) >> 15;  // South-South-East
    attacks |= (bitboard & !AB_FILE) >> 10; // West-South-West
    attacks |= (bitboard & !GH_FILE) >> 6;  // East-South-East

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

pub const fn mask_king_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let bitboard: u64 = 1u64 << square;

    // Horizontal and Vertical
    attacks |= (bitboard & !H_FILE) << 1;  // East
    attacks |= (bitboard & !A_FILE) >> 1;  // West
    attacks |= bitboard << 8;              // North
    attacks |= bitboard >> 8;              // South

    // Diagonals
    attacks |= (bitboard & !H_FILE) << 9;  // North-East
    attacks |= (bitboard & !A_FILE) << 7;  // North-West
    attacks |= (bitboard & !H_FILE) >> 7;  // South-East
    attacks |= (bitboard & !A_FILE) >> 9;  // South-West

    attacks
}

pub const fn generate_all_king_attacks() -> [u64; 64] {
    let mut table = [0; 64];
    let mut square = 0;

    while square < 64 {
        table[square] = mask_king_attacks(square as u8);
        square += 1;
    }

    table
}

