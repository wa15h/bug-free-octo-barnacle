use crate::Rng;

/// Everything the sim knows, as plain data. Two `State`s made from one seed and fed the same
/// inputs stay equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    /// Ticks run so far; during a tick, that tick's number, counting from 1.
    pub tick: u64,
    /// The sim's one RNG, seeded by the caller and kept here, so it saves and replays with the rest.
    pub rng: Rng,
}

impl State {
    pub fn new(seed: u64) -> State {
        State {
            tick: 0,
            rng: Rng::new(seed),
        }
    }
}

/// One thing the server asks of a tick. The server checks it first; the owning system checks again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    /// The cordon week from the server's calendar, the only way the week enters the sim.
    Calendar { season_seed: u64, week: u32 },
}

/// One thing a tick reports back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// An input the sim did not apply, returned as given. The state is as if it never came.
    Rejected { input: Input },
}

/// One step of `tick`. `run` gives each its rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum System {
    /// Counts the tick.
    TickCount,
    /// Takes `Input::Calendar`.
    Cordon,
}

/// Every system, once, in the order `tick` runs them.
pub const SYSTEM_ORDER: &[System] = &[System::TickCount, System::Cordon];

/// The hearth's systems, the only ones catch-up will run (D8), so a prefix of `SYSTEM_ORDER`.
/// Both systems so far are the hearth's; the expedition's will follow them in `SYSTEM_ORDER`.
pub const HEARTH_ORDER: &[System] = &[System::TickCount, System::Cordon];

/// One fixed step: runs each system in `SYSTEM_ORDER` and nothing else, and returns its events.
pub fn tick(state: &mut State, inputs: &[Input]) -> Vec<Event> {
    let mut events = Vec::new();
    for system in SYSTEM_ORDER {
        run(*system, state, inputs, &mut events);
    }
    events
}

fn run(system: System, state: &mut State, inputs: &[Input], events: &mut Vec<Event>) {
    match system {
        // A u64 count never fills at any tick rate. `saturating_add` keeps debug and release
        // alike where `+ 1` would panic in debug only.
        System::TickCount => state.tick = state.tick.saturating_add(1),
        System::Cordon => cordon(inputs, events),
    }
}

/// Until the cordon calendar lands (N-32) there is no stage table to check a week against, so
/// each `Calendar` input is rejected and changes nothing.
fn cordon(inputs: &[Input], events: &mut Vec<Event>) {
    for input in inputs {
        match input {
            Input::Calendar { .. } => events.push(Event::Rejected { input: *input }),
        }
    }
}
