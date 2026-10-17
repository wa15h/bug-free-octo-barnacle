use sim::{tick, Event, Input, Rng, State};

/// One random input; each `Input` kind gets an arm as it lands.
fn random_input(rng: &mut Rng) -> Input {
    Input::Calendar {
        season_seed: rng.next_u64(),
        week: rng.next_u64() as u32,
    }
}

/// Runs 1,000 ticks from `seed`, feeding each tick 0 to 3 random inputs drawn from an `Rng` with
/// the same seed. No rule takes an input yet, so each one must come back as one `Event::Rejected`,
/// in order, and leave `State` as it was apart from the tick count.
fn run(seed: u64) -> State {
    let mut state = State::new(seed);
    let mut draws = Rng::new(seed);
    for _ in 0..1_000 {
        let count = draws.next_u64() % 4;
        let inputs: Vec<Input> = (0..count).map(|_| random_input(&mut draws)).collect();
        let events = tick(&mut state, &inputs);
        let want: Vec<Event> = inputs
            .iter()
            .map(|&input| Event::Rejected { input })
            .collect();
        assert_eq!(events, want, "seed={seed} tick={}", state.tick);
    }
    let mut untouched = State::new(seed);
    untouched.tick = 1_000;
    assert_eq!(state, untouched, "seed={seed}");
    state
}

#[test]
fn random_inputs_for_1000_ticks_never_panic() {
    for seed in 0..16 {
        run(seed);
    }
}

#[test]
fn two_runs_from_one_seed_end_equal() {
    for seed in [0, 1, u64::MAX] {
        assert_eq!(run(seed), run(seed), "seed={seed}");
    }
}
