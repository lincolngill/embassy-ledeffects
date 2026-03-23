use crate::Strip;
use crate::effect::EffectIterator;
use defmt::Formatter;
use embassy_rp::clocks::RoscRng;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use heapless::spsc::Queue;
use smart_leds::{RGB8, colors};
const MAX_NUM_COMETS: usize = 16;

type HeadPos = usize;

#[derive(Copy, Clone)]
pub enum CometDirection {
    Up,
    Down,
}

impl defmt::Format for CometDirection {
    fn format(&self, fmt: Formatter) {
        match self {
            CometDirection::Up => defmt::write!(fmt, "Up"),
            CometDirection::Down => defmt::write!(fmt, "Down"),
        }
    }
}

pub struct Comet {
    next_head_pos: HeadPos,
    direction: CometDirection,
    ttl_pings: u8,
    alive: bool,
}

impl Comet {
    fn new(direction: CometDirection, ttl_pings: u8, strip_len: usize) -> Self {
        Comet {
            next_head_pos: match direction {
                CometDirection::Up => 0,
                CometDirection::Down => strip_len - 1,
            },
            direction,
            ttl_pings,
            alive: true,
        }
    }
    pub fn get_direction(self) -> CometDirection {
        self.direction
    }
    pub fn get_ttl_pings(self) -> u8 {
        self.ttl_pings
    }
}

pub struct Comets {
    comets: Queue<Comet, MAX_NUM_COMETS>,
    strip_len: usize,
}

impl Comets {
    pub fn new<const S: usize>(_: &Strip<S>) -> Self {
        Self {
            comets: Queue::new(),
            strip_len: S,
        }
    }
    pub fn launch(
        &mut self,
        direction: Option<CometDirection>,
        ttl_pings: Option<u8>,
    ) -> Result<(), Comet> {
        const DEF_DIRECTION: CometDirection = CometDirection::Up;
        const DEF_TTL_PINGS: u8 = 0;
        self.comets.enqueue(Comet::new(
            direction.unwrap_or(DEF_DIRECTION),
            ttl_pings.unwrap_or(DEF_TTL_PINGS),
            self.strip_len,
        ))
    }
    pub fn comet_cnt(&mut self) -> usize {
        self.comets.len()
    }
}

impl EffectIterator for Comets {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        // cooling
        const COOLDOWN_CHANCE_MASK: u8 = 0x03;
        for i in 0..S {
            if strip.leds[i] != colors::BLACK {
                if RoscRng.next_u32() as u8 & COOLDOWN_CHANCE_MASK == 0 {
                    let rn = RoscRng.next_u32();
                    strip.leds[i] = RGB8 {
                        r: strip.leds[i].r.saturating_sub(rn as u8),
                        g: strip.leds[i].g.saturating_sub((rn >> 8) as u8),
                        b: strip.leds[i].b.saturating_sub((rn >> 16) as u8),
                    };
                }
            }
        }
        // Update next_head_pos
        for c in self.comets.iter_mut() {
            if c.alive {
                strip.leds[c.next_head_pos] = colors::WHITE;
                match c.direction {
                    CometDirection::Up => {
                        c.next_head_pos += 1;
                        if c.next_head_pos == S {
                            if c.ttl_pings == 0 {
                                c.alive = false;
                            } else {
                                c.ttl_pings -= 1;
                                c.direction = CometDirection::Down;
                                c.next_head_pos -= 1;
                            }
                        }
                    }
                    CometDirection::Down => {
                        if c.next_head_pos == 0 {
                            if c.ttl_pings == 0 {
                                c.alive = false;
                            } else {
                                c.ttl_pings -= 1;
                                c.direction = CometDirection::Up;
                            }
                        } else {
                            c.next_head_pos -= 1;
                        }
                    }
                }
            }
        }
        // Deq oldest comet if it's dead
        if let Some(c) = self.comets.peek()
            && !c.alive
        {
            self.comets.dequeue().unwrap();
        }
        strip.inc_frame_cnt();
        Some(())
    }
}

pub enum CometsInMsg {
    Stop,
}
pub static COMETS_IN_MSG: Signal<ThreadModeRawMutex, CometsInMsg> = Signal::new();

pub enum CometsOutMsg {
    Launch,
    TaskEnded,
}
pub static COMETS_OUT_MSG: Signal<ThreadModeRawMutex, CometsOutMsg> = Signal::new();

// Randomly signal to launch a new comet
// Task stopped by sending a Stop msg on the COMETS_IN_MSGS signal.
#[embassy_executor::task]
pub async fn comets_task(min_delay_ms: Option<u32>, max_delay_ms: Option<u32>) {
    const DEF_MIN_DELAY_MS: u32 = 200;
    const DEF_MAX_DELAY_MS: u32 = 4000;
    const MIN_MIN_DELAY_MS: u32 = 20;
    let min_delay = min_delay_ms.unwrap_or(DEF_MIN_DELAY_MS);
    let max_delay = max_delay_ms.unwrap_or(DEF_MAX_DELAY_MS);
    assert!(
        min_delay > MIN_MIN_DELAY_MS,
        "min_delay must be > {}ms",
        MIN_MIN_DELAY_MS
    );
    assert!(
        max_delay > min_delay,
        "max_delay {} must be >= min_delay {}",
        max_delay,
        min_delay
    );
    loop {
        if COMETS_IN_MSG.signaled() {
            match COMETS_IN_MSG.wait().await {
                CometsInMsg::Stop => break,
            }
        }
        let delay = min_delay + (RoscRng.next_u32() % (max_delay - min_delay));
        Timer::after(Duration::from_millis(delay as u64)).await;
        COMETS_OUT_MSG.signal(CometsOutMsg::Launch);
    }
    COMETS_OUT_MSG.signal(CometsOutMsg::TaskEnded);
}
