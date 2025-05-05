#[derive(Clone, Copy)]
pub struct Engine {
    started_at: std::time::Instant,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            started_at: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> core::time::Duration {
        self.started_at.elapsed()
    }
}
