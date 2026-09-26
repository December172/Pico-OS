use core::ops::Add;
use core::ops::Sub;
use core::time::Duration;

pub const HZ : u64 = 1_000_000;

/// Presents a timestamp. Unit is in kt (kernel ticks)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    ticks: u64,
}

impl Time {
    pub fn getTicks(&self) -> u64 {
        return self.ticks;
    }

    pub const fn fromTicks(ticks: u64) -> Self {
        Self {
            ticks
        }
    }

    pub fn asMicroSecond(&self) -> u64 {
        return self.ticks;
    }

    pub fn asMilliSecond(&self) -> u64 {
        return self.ticks / 1_000;
    }

    pub fn asSecond(&self) -> u64 {
        return self.ticks / HZ;
    }
}

/// Time + Duration = Time
impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, rhs: Duration) -> Self::Output {
        let ticks = self.ticks
            .checked_add(rhs.as_micros() as u64)
            .expect("Time addition overflow");

        return Time::fromTicks(ticks);
    }
}

/// Time - Duration = Time
impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, rhs: Duration) -> Self::Output {
        let ticks = self.ticks
            .checked_sub(rhs.as_micros() as u64)
            .expect("Time subtraction underflow");

        return Time::fromTicks(ticks);
    }
}

/// Time - Time = Duration 
impl Sub for Time {
    type Output = Duration;

    /// <self> - <rhs>, duration starts from rhs.ticks, and ends at self.ticks
    fn sub(self, rhs: Time) -> Self::Output {
        if self.ticks < rhs.ticks {
            panic!("Time subtraction upperflow");
        }
        return Duration::from_micros(self.ticks - rhs.ticks);
    }
}