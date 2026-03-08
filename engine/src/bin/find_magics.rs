use std::fs::File;
use std::io::Write;
use engine::magics::{bishop_attacks_slow, mask_bishop_attacks, mask_rook_attacks, rook_attacks_slow, set_occupancy};

const ROOK_RELEVANT_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

const BISHOP_RELEVANT_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

fn main() {
    let mut file = File::create("src/magic_constants.rs")
        .expect("Failed to create magic_constants.rs file");

    writeln!(file, "pub const ROOK_MAGICS: [u64; 64] = [").unwrap();
    for square in 0..64 {
        let magic = find_magic(square, ROOK_RELEVANT_BITS[square as usize], true);
        writeln!(file, "     0x{magic:016X}, // Square {square}").unwrap();
    }
    writeln!(file, "];\n").unwrap();

    writeln!(file, "pub const BISHOP_MAGICS: [u64; 64] = [").unwrap();
    for square in 0..64 {
        let magic = find_magic(square, BISHOP_RELEVANT_BITS[square as usize], false);
        writeln!(file, "     0x{magic:016X}, // Square {square}").unwrap();
    }
    writeln!(file, "];").unwrap();

    println!("Successfully generated magic_constants.rs!");
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
