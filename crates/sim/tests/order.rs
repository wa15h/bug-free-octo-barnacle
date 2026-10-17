use sim::{System, HEARTH_ORDER, SYSTEM_ORDER};

/// Every `System`. The match in `order_is_complete` has no `_` arm, so a new `System` stops this
/// file compiling until it is added there and to this list.
const EVERY_SYSTEM: [System; 2] = [System::TickCount, System::Cordon];

#[test]
fn order_is_complete() {
    for system in SYSTEM_ORDER {
        match system {
            System::TickCount | System::Cordon => {}
        }
    }
    for system in EVERY_SYSTEM {
        let times = SYSTEM_ORDER.iter().filter(|s| **s == system).count();
        assert_eq!(times, 1, "{system:?} is in SYSTEM_ORDER {times} times");
    }
    assert_eq!(SYSTEM_ORDER.len(), EVERY_SYSTEM.len());
    assert!(
        SYSTEM_ORDER.starts_with(HEARTH_ORDER),
        "HEARTH_ORDER {HEARTH_ORDER:?} is not a prefix of SYSTEM_ORDER {SYSTEM_ORDER:?}"
    );
}
