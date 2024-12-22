use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub mod accelerometer;
pub mod randr;
pub mod transform;

pub struct TerminationSignal {
    terminate: Arc<AtomicBool>,
}

impl TerminationSignal {
    pub fn new() -> Self {
        Self {
            terminate: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn register(self) -> Result<Self, std::io::Error> {
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&self.terminate))?;
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&self.terminate))?;
        Ok(self)
    }

    pub fn should_terminate(&self) -> bool {
        self.terminate.load(Ordering::Relaxed)
    }
}
