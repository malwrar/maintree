use std::fmt;
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct RenderStats {
    start_time: Instant,
    end_time: Option<Instant>,
}

impl RenderStats {
    pub fn begin() -> Self {
        Self {
            start_time: Instant::now(),
            end_time: None
        }
    }

    pub fn end(&mut self) {
        if self.end_time.is_none() {
            self.end_time = Some(Instant::now());
        }
    }
}

impl fmt::Display for RenderStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cur_time = match self.end_time {
            Some(time) => time,
            None => Instant::now()
        };

        write!(f, "RenderStats: ");
        write!(f, "time={} ", (cur_time - self.start_time).as_secs())
    }
}
