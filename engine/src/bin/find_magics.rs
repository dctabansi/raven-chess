use engine::constants::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS};
use engine::magics::{
    bishop_attacks_slow, mask_bishop_attacks,
    mask_rook_attacks, rook_attacks_slow, set_occupancy
};
use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    // write_to_file("src/constants.rs");
    print_results();
}

#[derive(Debug)]
struct Prng{
    state: u64,
}

impl Prng{
    fn new(seed: u64) -> Self{
        Self { state: seed }
    }

    fn rand64(&mut self) -> u64{
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;

        self.state
    }

    fn random_sparse_u64(&mut self) -> u64{
        self.rand64() & self.rand64() & self.rand64()
    }
}

fn find_magic(square: u8, relevant_bits: u8, is_rook: bool) -> u64{
    let mask = if is_rook {
        mask_rook_attacks(square)
    } else {
        mask_bishop_attacks(square)
    };

    let num_permutations = 1 << relevant_bits;

    let mut occupancies = vec![0u64; num_permutations];
    let mut attacks = vec![0u64; num_permutations];

    let mut used_attacks = vec![0u64; 4096];

    for index in 0..num_permutations {
        occupancies[index] = set_occupancy(index, mask);
        attacks[index] = if is_rook {
            rook_attacks_slow(square, occupancies[index])
        } else {
            bishop_attacks_slow(square, occupancies[index])
        };
    }

    let mut prng = Prng::new(1010);

    for _ in 0..100_000_000 {
        let magic = prng.random_sparse_u64();

        if (mask.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            continue;
        }

        used_attacks.fill(0);
        let mut fail = false;

        for i in 0..num_permutations {
            let index = ((occupancies[i].wrapping_mul(magic)) >> (64 - relevant_bits)) as usize;

            if used_attacks[index] == 0 {
                used_attacks[index] = attacks[i];
            } else if used_attacks[index] != attacks[i] {
                fail = true;
                break;
            }
        }

        if !fail {
            return magic;
        }
    }

    panic!("Failed to find magic number for square {square} after 100M tries!");
}

fn write_to_file(path: &str) {
    // Copy the arrays to local variables before doing any file operations
    let rook_bits = ROOK_RELEVANT_BITS;
    let bishop_bits = BISHOP_RELEVANT_BITS;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Failed to open or create {} file", path));

    // write!(file, "{}", get_relevant_bits_string()).unwrap();

    writeln!(file, "\n\npub const BISHOP_MAGICS_ARRAY: [u64; 64] = [").unwrap();
    for square in 0..64 {
        let magic = find_magic(square, bishop_bits[square as usize], false);
        writeln!(file, "     0x{magic:016X}, // Square {square}").unwrap();
    }
    writeln!(file, "];\n").unwrap();

    writeln!(file, "pub const ROOK_MAGICS_ARRAY: [u64; 64] = [").unwrap();
    for square in 0..64 {
        let magic = find_magic(square, rook_bits[square as usize], true);
        writeln!(file, "     0x{magic:016X}, // Square {square}").unwrap();
    }
    writeln!(file, "];").unwrap();

    println!("Successfully appended to {path}!");
}

fn print_results() {
    // print!("{}", get_relevant_bits_string());

    println!("pub const BISHOP_MAGICS_ARRAY: [u64; 64] = [");
    for square in 0..64 {
        let magic = find_magic(square, BISHOP_RELEVANT_BITS[square as usize], false);
        println!("     0x{magic:016X}, // Square {square}");
    }
    println!("];\n");

    println!("pub const ROOK_MAGICS_ARRAY: [u64; 64] = [");
    for square in 0..64 {
        let magic = find_magic(square, ROOK_RELEVANT_BITS[square as usize], true);
        println!("     0x{magic:016X}, // Square {square}");
    }
    println!("];");
}

// fn get_relevant_bits_string() -> &'static str {
//     "pub const BISHOP_RELEVANT_BITS: [u8; 64] = [
//      6, 5, 5, 5, 5, 5, 5, 6,
//      5, 5, 5, 5, 5, 5, 5, 5,
//      5, 5, 7, 7, 7, 7, 5, 5,
//      5, 5, 7, 9, 9, 7, 5, 5,
//      5, 5, 7, 9, 9, 7, 5, 5,
//      5, 5, 7, 7, 7, 7, 5, 5,
//      5, 5, 5, 5, 5, 5, 5, 5,
//      6, 5, 5, 5, 5, 5, 5, 6,
// ];
//
// pub const ROOK_RELEVANT_BITS: [u8; 64] = [
//      12, 11, 11, 11, 11, 11, 11, 12,
//      11, 10, 10, 10, 10, 10, 10, 11,
//      11, 10, 10, 10, 10, 10, 10, 11,
//      11, 10, 10, 10, 10, 10, 10, 11,
//      11, 10, 10, 10, 10, 10, 10, 11,
//      11, 10, 10, 10, 10, 10, 10, 11,
//      11, 10, 10, 10, 10, 10, 10, 11,
//      12, 11, 11, 11, 11, 11, 11, 12,
// ];\n\n"
// }
