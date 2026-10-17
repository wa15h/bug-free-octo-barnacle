use sim::{Fixed, FixedError, Rng, SCALE};

/// Published in rand_xoshiro 0.6.0's `reference` tests, made with the reference C code at
/// https://prng.di.unimi.it: `src/splitmix64.rs`, SplitMix64's first four outputs from seed
/// 1477776061723855037; `src/xoshiro256starstar.rs`, the first ten outputs from state 1, 2, 3, 4.
#[test]
fn rng_matches_published_reference_outputs() {
    let seeded = [
        1985237415132408290,
        2979275885539914483,
        13511426838097143398,
        8488337342461049707,
    ];
    assert_eq!(Rng::new(1477776061723855037).s, seeded);
    let mut rng = Rng { s: [1, 2, 3, 4] };
    let expected = [
        11520,
        0,
        1509978240,
        1215971899390074240,
        1216172134540287360,
        607988272756665600,
        16172922978634559625,
        8476171486693032832,
        10595114339597558777,
        2904607092377533576,
    ];
    for want in expected {
        assert_eq!(rng.next_u64(), want);
    }
}

/// The exact answer as a `Fixed`, or `Overflow` when it is outside the `i64` range.
fn exact(wide: i128) -> Result<Fixed, FixedError> {
    match i64::try_from(wide) {
        Ok(raw) => Ok(Fixed(raw)),
        Err(_) => Err(FixedError::Overflow),
    }
}

/// Each operation equals `i128` math, rounded toward zero, or the `Err` that math predicts.
fn check(x: Fixed, y: Fixed) {
    let (a, b, scale) = (i128::from(x.0), i128::from(y.0), i128::from(SCALE));
    assert_eq!(x.checked_add(y), exact(a + b), "{x:?} + {y:?}");
    assert_eq!(x.checked_sub(y), exact(a - b), "{x:?} - {y:?}");
    assert_eq!(x.checked_mul(y), exact(a * b / scale), "{x:?} * {y:?}");
    match b {
        0 => assert_eq!(x.checked_div(y), Err(FixedError::DivideByZero), "{x:?} / 0"),
        _ => assert_eq!(x.checked_div(y), exact(a * scale / b), "{x:?} / {y:?}"),
    }
}

/// Every edge-value pair, then 10,000 pairs from seed 1: random words shifted right 0 to 63 bits.
#[test]
fn fixed_equals_i128_math_or_err() {
    let edges = [i64::MIN, i64::MIN + 1, -SCALE, -1, 0, 1, SCALE, i64::MAX];
    for a in edges {
        for b in edges {
            check(Fixed(a), Fixed(b));
        }
    }
    let mut rng = Rng::new(1);
    for _ in 0..10_000 {
        let a = (rng.next_u64() as i64) >> (rng.next_u64() % 64);
        let b = (rng.next_u64() as i64) >> (rng.next_u64() % 64);
        check(Fixed(a), Fixed(b));
    }
}
