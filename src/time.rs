use chrono::prelude::*;

#[derive(Debug, Clone,Copy)]
pub(crate) struct Time {
    pub(crate) miliseconds: i64,
    pub(crate) second: i64,
    pub(crate) minute: i64,
}

#[derive(Debug, Clone,Copy)]
pub(crate) struct Timer {
    pub(crate) beginng: chrono::DateTime<Utc>,
    pub(crate) time_elapsed: Time,
}
impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            beginng: Utc::now(),
            time_elapsed: Time {
                miliseconds: 0,
                second: 0,
                minute: 0,
            },
        }
    }
    pub(crate) fn start(&mut self) {
        self.beginng = Utc::now();
    }


    pub(crate) fn update_timer(&mut self, now: chrono::DateTime<Utc>) {
        let elapsed = now - self.beginng;
        self.time_elapsed = Time {
            miliseconds: elapsed.num_milliseconds(),
            second: elapsed.num_seconds(),
            minute: elapsed.num_minutes(),
        }
    }
}
