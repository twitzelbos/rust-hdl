use crate::synchronous::Synchronous;

struct Counter {}

impl Synchronous for Counter {
    type State = u32;
    type Input = bool;
    type Output = u32;
    fn update(&self, state_q: u32, enable: bool) -> (u32, u32) {
        let state_d = if enable { state_q + 1 } else { state_q };
        (state_q, state_d)
    }
}

// Count to 1e9
#[test]
fn test_count_to_1e9() {
    let mut state = 0_u32;
    let mut output = 0_u32;
    let now = std::time::Instant::now();
    let counter = Counter {};
    for cycle in 0..1_000_000_000 {
        (output, state) = counter.update(state, cycle % 2 == 0);
    }
    println!(
        "Final state: {state:?}, elapsed time {}",
        now.elapsed().as_millis()
    );
}
