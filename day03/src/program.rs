use std::time::{Duration, Instant};

pub struct Program {
    events: Vec<Event>,
    state: State,
}

impl Program {
    pub fn run(&mut self) {
        while !self.state.exit {
            for e in self.events.iter_mut() {
                e.try_run(&mut self.state);
            }
        }
    }

    pub fn register(&mut self, event: Event) {
        self.events.push(event);
    }
}

impl Default for Program {
    fn default() -> Self {
        Program {
            events: Vec::new(),
            state: State::default(),
        }
    }
}

pub struct Event {
    pub func: fn(&mut State),
    pub first_call: Duration,
    pub interval: Option<Duration>,
    pub times_called: u32, // Always initialize at 0
}

impl Event {
    /// Runs event if it can be run.
    fn try_run(&mut self, state: &mut State) {
        // If repeating event.
        if let Some(call_interval) = self.interval {
            if Instant::now()
                >= state.program_start() + self.first_call + (call_interval * self.times_called)
            {
                (self.func)(state);
                self.times_called += 1;
            }
        } else {
            if self.times_called > 0 {
                return;
            }

            if Instant::now() >= state.program_start() + self.first_call {
                (self.func)(state);
                self.times_called += 1;
            }
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            func: |&mut _| {},          // Do nothing.
            first_call: Duration::ZERO, // Start at beginning.
            interval: None,             // Don't repeat.
            times_called: 0,            // Has never been called before. (Do not edit!)
        }
    }
}

#[derive(Copy, Clone)]
pub struct State {
    pub rain_pos: [(u16, u16); 15],
    program_start: Instant,
    pub exit: bool,
}

impl State {
    pub fn program_start(&self) -> Instant {
        self.program_start
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            rain_pos: [(1, 1); 15],
            program_start: Instant::now(),
            exit: false,
        }
    }
}

pub fn initialize_program() -> Program {
    Program {
        events: Vec::new(),
        state: State::default(),
    }
}
