use std::fmt;
use std::time::{Instant, Duration};
use std::borrow::Cow;

type CowStr = Cow<'static, str>;

/// A multi-purpose timer, for debugging. When an instance is stopped or
/// goes out of scope, the label and the elapsed time is printed to stderr.
pub struct BlockTimer {
    label: CowStr,
    start: Instant,
    stopped: bool,
}

/// A struct which implements fmt::Display to provide a human-readable
/// description of a `Duration`.
pub struct PrettyDuration {
    secs: u64,
    millis: u64,
    micros: u64,
    nanos: u64,
}

impl BlockTimer {
    pub fn new<S: Into<CowStr>>(label: S) -> Self {
        BlockTimer {
            label: label.into(),
            start: Instant::now(),
            stopped: false,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
        let elapsed = self.start.elapsed();
        let d = PrettyDuration::new(elapsed);
        eprintln!("{}: {}", self.label, d);
    }
}

impl Drop for BlockTimer {
    fn drop(&mut self) {
        if self.stopped { return }
        self.stop();
    }
}

impl PrettyDuration {
    pub fn new(d: Duration) -> Self {
        let d = nanos_from_duration(d);
        let secs = d / 1_000_000_000;
        let d = d - secs * 1_000_000_000;
        let millis = d / 1_000_000;
        let d = d - millis * 1_000_000;
        let micros = d / 1_000;
        let nanos = d - micros * 1_000;
        PrettyDuration { secs, millis, micros, nanos }
    }
}

impl fmt::Display for PrettyDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.secs > 0 {
            write!(f, "{}.{}s", self.secs, self.millis / 100)
        } else if self.millis > 0 {
            write!(f, "{}.{}ms", self.millis, self.micros / 100)
        } else if self.micros > 0 {
            write!(f, "{}us", self.micros)
        } else {
            write!(f, "{}ns", self.nanos)
        }
    }
}

fn nanos_from_duration(d: Duration) -> u64 {
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
