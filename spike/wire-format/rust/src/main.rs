// W0-10 wire-format spike (ADR 0004). Lives only on claude/spike-wire-format; never merged.
// Builds the cut-scope and 4x snapshots, times the FlatBuffers encode (median of five runs, each
// 10,000 encodes after 1,000 warm-up), and writes each snapshot's bytes plus a line of
// `entities bytes encode_us checksum` into the directory given as the one argument, for the C#
// decoder to read and check against.

#[allow(warnings, clippy::all)]
#[path = "../generated/snapshot_generated.rs"]
mod snapshot_generated;

use flatbuffers::FlatBufferBuilder;
use snapshot_generated::spike::{Entity, EntityArgs, Snapshot, SnapshotArgs};
use std::hint::black_box;
use std::time::Instant;

const WARMUP: usize = 1_000;
const TIMED: usize = 10_000;
const RUNS: usize = 5;
const SEED: u64 = 1;

/// Players, boars, settlers, buildings, ledger lines, cordon stage (guesses, ADR 0004).
const CUT: [usize; 6] = [2, 8, 6, 20, 12, 1];
/// 4x quadruples all but players and the cordon stage.
const FOUR_X: [usize; 6] = [2, 32, 24, 80, 48, 1];

/// SplitMix64: a fixed, dependency-free generator, so the values repeat on every machine.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Ids 1 to n in order; five values each, uniform over the `i32` range, from `SEED`.
fn entities(counts: &[usize; 6]) -> Vec<(u32, [i64; 5])> {
    let n = counts.iter().sum::<usize>() as u32;
    let mut rng = SplitMix64(SEED);
    (1..=n)
        .map(|id| {
            (
                id,
                std::array::from_fn(|_| i64::from(rng.next() as u32 as i32)),
            )
        })
        .collect()
}

/// The checksum the C# decoder must reproduce by reading every field in list order.
fn checksum(ents: &[(u32, [i64; 5])]) -> u64 {
    let mut c: u64 = 0;
    for (id, values) in ents {
        c = c.wrapping_mul(31).wrapping_add(u64::from(*id));
        for v in values {
            c = c.wrapping_mul(31).wrapping_add(*v as u64);
        }
    }
    c
}

fn encode<'b>(b: &'b mut FlatBufferBuilder<'static>, ents: &[(u32, [i64; 5])]) -> &'b [u8] {
    b.reset();
    let offsets: Vec<_> = ents
        .iter()
        .map(|(id, v)| {
            let args = EntityArgs {
                id: *id,
                v0: v[0],
                v1: v[1],
                v2: v[2],
                v3: v[3],
                v4: v[4],
            };
            Entity::create(b, &args)
        })
        .collect();
    let list = b.create_vector(&offsets);
    let root = Snapshot::create(
        b,
        &SnapshotArgs {
            entities: Some(list),
        },
    );
    b.finish(root, None);
    b.finished_data()
}

fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(f64::total_cmp);
    xs[xs.len() / 2]
}

fn main() {
    let out = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("usage: wire-format-spike <out dir>"),
    );
    std::fs::create_dir_all(&out).expect("create out dir");
    let mut b = FlatBufferBuilder::new();
    for (size, counts) in [("cut", &CUT), ("4x", &FOUR_X)] {
        let ents = entities(counts);
        let mut runs = Vec::new();
        for _ in 0..RUNS {
            for _ in 0..WARMUP {
                black_box(encode(&mut b, black_box(&ents)).len());
            }
            let start = Instant::now();
            for _ in 0..TIMED {
                black_box(encode(&mut b, black_box(&ents)).len());
            }
            runs.push(start.elapsed().as_secs_f64() * 1e6 / TIMED as f64);
        }
        let bytes = encode(&mut b, &ents).to_vec();
        let sum = checksum(&ents);
        let us = median(runs.clone());
        println!(
            "rust {size}: {} entities, {} bytes, encode us runs {runs:.2?}, median {us:.1}",
            ents.len(),
            bytes.len()
        );
        std::fs::write(out.join(format!("{size}.bin")), &bytes).expect("write bin");
        let line = format!("{} {} {us:.1} {sum}\n", ents.len(), bytes.len());
        std::fs::write(out.join(format!("{size}.txt")), line).expect("write txt");
    }
}
