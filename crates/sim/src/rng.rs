/// The sim's one RNG: `xoshiro256**` from <https://prng.di.unimi.it/xoshiro256starstar.c>, seeded
/// by the caller. `s` is its state, named as there. A state of all zeros never changes, and `new`
/// cannot make one: the four SplitMix64 outputs it uses all differ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rng {
    pub s: [u64; 4],
}

impl Rng {
    /// Fills the state from `seed` with SplitMix64, as the reference advises (its `splitmix64.c`).
    pub fn new(seed: u64) -> Rng {
        let mut x = seed;
        let mut s = [0; 4];
        for word in &mut s {
            x = x.wrapping_add(0x9e3779b97f4a7c15);
            let z = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            let z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            *word = z ^ (z >> 31);
        }
        Rng { s }
    }

    /// Returns the next number and steps the state, line for line as the reference `next()`.
    pub fn next_u64(&mut self) -> u64 {
        let s = &mut self.s;
        let result = s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        result
    }
}
