// Need Instant, Timer, Duration
// Clock at 1MHZ


use super::{ TICK_HZ, GCD_1K, GCD_1M, GCD_1G };

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    ticks: u64
}

impl Instant {
    pub const MIN: Instant = Instant { ticks: u64::MIN };
    pub const MAX: Instant = Instant { ticks: u64::MAX };

    #[inline]
    pub fn now() -> Self {
        unsafe {
            loop {

                let hi = core::ptr::read_volatile((0x3F00_3000 + 0x8) as *const u32);
                let lo = core::ptr::read_volatile((0x3F00_3000 + 0x4) as *const u32);
                let hi2 = core::ptr::read_volatile((0x3F00_3000 + 0x8) as *const u32);
                if hi == hi2 {
                    return Instant {
                        ticks: (hi as u64) << 32 | (lo as u64)
                    }
                }
            }
        }
    }

    pub const fn from_ticks(ticks: u64) -> Self {
        Self { ticks }
    }

    pub const fn to_u64(&self) -> u64 {
        self.ticks
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    pub(crate) ticks: u64
}

impl Duration {
    pub const MIN: Duration = Duration { ticks: u64::MIN };
    pub const MAX: Duration = Duration { ticks: u64::MAX };

    pub const fn from_secs(secs: u64) -> Self {
        Duration { ticks: secs * TICK_HZ }
    }

    pub const fn from_millis(millis: u64) -> Duration {
        Duration {
            ticks: millis * (TICK_HZ / GCD_1K).div_ceil(1000 / GCD_1K),
        }
    }

    pub const fn from_micros(micros: u64) -> Duration {
        Duration {
            ticks: micros * (TICK_HZ / GCD_1M).div_ceil(1_000_000 / GCD_1M),
        }
    }

    pub const fn from_nanos(micros: u64) -> Duration {
        Duration {
            ticks: micros * (TICK_HZ / GCD_1G).div_ceil(1_000_000_000 / GCD_1G),
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timer;

impl Timer {
    #[inline(always)]
    pub fn delay(duration: Duration) {
        let after = Instant::now().ticks + duration.ticks;

        while after > Instant::now().ticks {}
    }
}