/// Implements a basic monocular simultaneous localization & mapping system for realtime AR.

pub mod state;

pub struct SlamBuilder {
    state: state::SlamState;
}

impl SlamBuilder {
    fn new() -> Self {
        state: state::SlamState,
    }

    fn build(self) -> Slam {
        Slam {
            state: self.state,
        }
    }
}

pub struct Slam {
    state: state::SlamState,
}

impl Slam {
    fn new() -> SlamBuilder {
        SlamBuilder::new()
    }
}