use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use rand::Rng;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

type Bitboard = u64;
type Square = u8;

struct Magic {
    mask: Bitboard,
    magic: Bitboard,
    shift: u8,
    attacks: Vec<Bitboard>,
}

const fn square_rank(sq: Square) -> i8 {
    sq as i8 / 8
}

const fn square_file(sq: Square) -> i8 {
    sq as i8 % 8
}

// Check if a square is on board
const fn on_board(rank: i8, file: i8) -> bool {
    rank >= 0 && rank < 8 && file >= 0 && file < 8
}

const DIRECTIONS: &[(i8, i8)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
// const DIRECTIONS: &[(i8, i8)] = &[(1, 1), (1, -1), (-1, 1), (-1, -1)];

const MASKS: [Bitboard; 64] = {
    let mut masks = [0; 64];
    let mut i = 0;
    while i < 64 {
        let square = i as Square;
        let mut mask = 0;
        let start_rank = square_rank(square);
        let start_file = square_file(square);

        let mut j = 0;
        while j < DIRECTIONS.len() {
            let (dx, dy) = DIRECTIONS[j];
            let mut r = start_rank + dy;
            let mut f = start_file + dx;

            while on_board(r, f) {
                let sq = (r as usize) * 8 + (f as usize);
                if sq as u8 != square {
                    mask |= 1u64 << sq;
                }
                r += dy;
                f += dx;
            }

            j += 1;
        }
        masks[i] = mask;
        i += 1;
    }
    masks
};

// Generate all blocker permutations for mask bits
fn generate_blocker_permutations(mask: Bitboard) -> Vec<Bitboard> {
    let bits = mask.count_ones();

    (0..(1 << bits)).map(move |i| {
        let mut result = 0;
        let mut temp = mask;
        for j in 0..bits {
            let lsb = temp.trailing_zeros();
            result |= (((i >> j) & 1) as u64) << lsb;
            temp ^= 1 << lsb;
        }
        result
    }).collect()
}

pub static BLOCKER_PERMS: Lazy<Vec<Vec<Bitboard>>> = Lazy::new(|| {
    (0..64)
        .map(|sq| generate_blocker_permutations(MASKS[sq]))
        .collect()
});

// Generate attacks for given blockers and directions (naive)
fn gen_attacks_naive(square: Square, blockers: Bitboard) -> Bitboard {
    let mut attacks = 0;
    let start_rank = square_rank(square);
    let start_file = square_file(square);

    for &(dx, dy) in DIRECTIONS {
        let mut r = start_rank + dy;
        let mut f = start_file + dx;

        while on_board(r, f) {
            let sq = (r as usize) * 8 + (f as usize);
            attacks |= 1u64 << sq;

            if blockers & (1u64 << sq) != 0 {
                break;
            }

            r += dy;
            f += dx;
        }
    }

    attacks
}

// Try to find magic number for a square with given mask and directions
fn try_find_magic(
    square: Square,
    bits: usize,
    rng: &mut rand::rngs::ThreadRng,
    attempts: usize,
) -> Option<(Bitboard, Vec<Bitboard>)> {
    let blockers = &BLOCKER_PERMS[square as usize];
    let attacks: Vec<Bitboard> = blockers
        .iter()
        .map(|&b| gen_attacks_naive(square, b))
        .collect();

    for _ in 0..attempts {
        let magic = rng.random::<u64>() & rng.random::<u64>()
                 >> rng.random_range(0..16);

        const INVALID: u64 = 0xdeadbeefdeadbeef;
        let mut used = vec![INVALID; 1 << bits];
        let mut success = true;

        for (i, &b) in blockers.iter().enumerate() {
            // I forgot why I did this but I think it fixed something:           & (size - 1)
            let index = ((b.wrapping_mul(magic)) >> (64 - bits)) as usize;
            let attack = attacks[i];

            if used[index] == INVALID {
                used[index] = attack;
            } else if used[index] != attack {
                success = false;
                break;
            }
        }

        if success {
            return Some((magic, used));
        }
    }

    None
}

// Generate all magics for 64 squares given directions
fn generate_all_magics() -> Vec<Magic> {
    let total_entries = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicUsize::new(0));
    let start_time = Arc::new(Instant::now());

    let magics: Vec<Magic> = (0..64)
        .into_par_iter()
        .with_min_len(4)
        .map_init(
            || rand::rng(),
            |rng, square| {
                let mask = MASKS[square as usize];

                let bits_upper_bound = mask.count_ones() as usize;
                let mut found: Option<(usize, u64, Vec<u64>)> = None;

                for bits in 1..=bits_upper_bound {
                    if let Some((magic, attacks)) = try_find_magic(square, bits, rng, 10_000_000) {
                        found = Some((bits, magic, attacks));
                        break; // Stop at the first successful smallest table
                    }
                }

                let (bits, magic, attacks) = found.expect("❌ Failed to find magic for square");

                total_entries.fetch_add(attacks.len(), Ordering::Relaxed);

                let completed = completed.fetch_add(1, Ordering::Relaxed) + 1;
                let total_bytes = total_entries.load(Ordering::Relaxed) * std::mem::size_of::<Bitboard>();
                let total_mb = total_bytes as f64 / (1000.0 * 1000.0);
                let total_kb = total_bytes as f64 / (1000.0);
                let avg_mb = total_mb / completed as f64;
                let avg_kb = total_kb / completed as f64;

                print!(
                    "\x1b[2K\rProgress: {}/64 | Total: {:.2} MB / {:.2} KB | Avg/Square: {:.3} MB / {:.3} KB | Time: {:.3}s",
                    completed,
                    total_mb,
                    total_kb,
                    avg_mb,
                    avg_kb,
                    start_time.elapsed().as_secs_f64()
                );
                std::io::stdout().flush().unwrap();

                Magic {
                    mask,
                    magic,
                    shift: (64 - bits) as u8,
                    attacks,
                }
            }
        )
        .collect();

    println!();

    magics
}

// fn magic_sliding_attacks(blockers: Bitboard, m: &Magic) -> Bitboard {
//     let masked = blockers & m.mask;
//     let index = ((masked.wrapping_mul(m.magic)) >> m.shift) as usize;
//     m.attacks[index]
// }

// Writes results to output.rs
fn write_magics_to_file(magics: Vec<Magic>) {
    let output_path = Path::new(file!())
        .parent()
        .unwrap()
        .join("output.rs");

    let file = File::create(&output_path)
        .expect("Failed to create output.rs");
    let mut writer = BufWriter::new(file);

    write!(writer, "use super::{{Bitboard, Magic}};\n\n").unwrap();
    write!(writer, "pub const MAGICS: [Magic; 64] = [\n").unwrap();
    for (i, magic) in magics.iter().enumerate() {
        write!(
            writer,
            "    Magic {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, attacks: &ATTACK_TABLE_{} }},\n",
            magic.mask, magic.magic, magic.shift, i
        ).unwrap();
    }
    write!(writer, "];\n\n").unwrap();

    for (i, magic) in magics.iter().enumerate() {
        write!(writer, "const ATTACK_TABLE_{}: [Bitboard; {}] = [", i, magic.attacks.len()).unwrap();
        for attack in magic.attacks.iter() {
            write!(writer, "0x{:016X},", attack).unwrap();
        }
        write!(writer, "];\n").unwrap();
    }

    writer.flush().expect("Failed to flush writer");
}

fn main() {
    println!("Generating magics...");
    let magics = generate_all_magics();
    println!("Writing results to output.rs");
    write_magics_to_file(magics);
}
