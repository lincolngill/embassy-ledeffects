//! The Comets effect simulates multiple Comets travelling up and down the LED [crate::Strip].
//!
//! Each Comet has a bright (White) head with a trailing tail of diminishing red, green and blue values.
//! Each Comet travels (pings) up and down the LED [crate::Strip] a number of times before dying and disappearing.
//!
//! The effect does nothing until the [Comets::launch] function is invoked to start a new Comet.
//! Multiple Comets can be in-flight at the same time.
//!
//! The optional [launcher_task] can be used to randomly signal to launch a new Comet.
//! The timing of the random launch signals is controlled by a minumim and maximum loop wait delay.
//!
//! The [stop_launcher_task] function will signal (and await) for the launcher task to stop.
//!
//! The [launch_signaled] function is a non-blocking way to check if a launch has been signaled.
//!
//! # Examples
//! Refer to `examples/comets.rs` for a simply example of the Comets effect.
//!
//! Refer to `examples/strip_buttons.rs` for an example that launches a Comet both randomly via the [launcher_task] and by a button press.
//! The launch also selects a random starting direction and a random number of time-to-live pings.
use crate::Strip;
use crate::effect::EffectIterator;
use defmt::Formatter;
use embassy_rp::clocks::RoscRng;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, TimeoutError, with_timeout};
use heapless::spsc::Queue;
use smart_leds::{RGB8, colors};

/// The maximum number of in-flight Comets.
///
/// The Comets effect uses a fixed size heapless::spsc::Queue to store in-flight Comet objects.
pub const MAX_NUM_COMETS: usize = 16;

type HeadPos = usize;

/// Direction of comet travel. Up or Down the LED [crate::Strip].
#[derive(Copy, Clone)]
pub enum CometDirection {
    /// Comet position on the [crate::Strip] is incrementing.
    Up,
    /// Comet position on the [crate::Strip] is decrementing.
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

/// The Comet struct represents a single comet.
///
/// Comets travel up or down the strip, with a time-to-live number of pings back and forth.
/// Once the time-to-live pings is exausted the comet dies and is removed from the Comets effect.
///
/// Comets are created by calling the `launch` method on the Comets object.
struct Comet {
    next_head_pos: HeadPos,
    direction: CometDirection,
    ttl_pings: u8,
    alive: bool,
}

impl Comet {
    /// Create a new Comet.
    ///
    /// # Arguments
    /// * `direction: CometDirection` - Initial direction of travel.
    /// * `ttl_pings` - Number of strip traversals to run before the Comet dies.
    /// * `strip_len` - The Comets starting position, when traveling down the strip.
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
}

/// The Comets struct contains all in-flight Comets.
///
/// A maximum of [`MAX_NUM_COMETS`] can be in-flight at once.
pub struct Comets {
    comets: Queue<Comet, MAX_NUM_COMETS>,
    strip_len: usize,
}

impl Comets {
    /// Create a new Comets effect object.
    ///
    /// # Arguments
    /// * `_` - The LED Strip reference used to determine the strip length for the Comets effect.
    pub fn new<const S: usize>(_: &Strip<S>) -> Self {
        Self {
            comets: Queue::new(),
            strip_len: S,
        }
    }
    /// Launch a new Comet.
    ///
    /// Adds a new Comet to the Comets effect
    ///
    /// # Arguments
    /// * `direction` - Initial direction of travel. Defaults: [CometDirection::Up]
    /// * `ttl_pings` - Number of times the Comet will ping back in the other direction, before it dies, when it reaches the end of the strip. Default: 0
    ///
    /// Returns `Ok(())` if the new Comet is sucessfully launched.
    ///
    /// # Errors
    /// Returns `Err("Too many Comets")` if the in-flight Comets queue is full.
    pub fn launch(
        &mut self,
        direction: Option<CometDirection>,
        ttl_pings: Option<u8>,
    ) -> Result<(), &str> {
        const DEF_DIRECTION: CometDirection = CometDirection::Up;
        const DEF_TTL_PINGS: u8 = 0;
        match self.comets.enqueue(Comet::new(
            direction.unwrap_or(DEF_DIRECTION),
            ttl_pings.unwrap_or(DEF_TTL_PINGS),
            self.strip_len,
        )) {
            Ok(()) => Ok(()),
            Err(_) => Err("Too many Comets"),
        }
    }
    /// Get the current number of in-flight Comets.
    pub fn comet_cnt(&mut self) -> usize {
        self.comets.len()
    }
}

impl EffectIterator for Comets {
    /// Generates the next frame of the Comets effect by updating the position of each in-flight Comet and removing any that have expired.
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
            self.comets
                .dequeue()
                .expect("Comet queue peeked but failed to dequeue");
        }
        strip.inc_frame_cnt();
        Some(())
    }
}

/// Messages to recieve from, or send to, the Comet launcher task.
enum TaskMsg {
    Stop,
    Ended,
}
static IN_MSG: Signal<ThreadModeRawMutex, TaskMsg> = Signal::new();
static OUT_MSG: Signal<ThreadModeRawMutex, TaskMsg> = Signal::new();
static LAUNCH: Signal<ThreadModeRawMutex, ()> = Signal::new();

/// The embassy launcher_task generates Launch signals at random intervals.
///
/// # Arguments
/// * `min_delay_ms` - The minimum millisecond delay between Launch signals. Default: 200 Minimum: 20
/// * `max_delay_ms` - The maximum millisecond dealy between Launch signals. Default: 4000
///
/// It is stopped by calling [stop_launcher_task] function
///
/// # Example
/// ```rust
/// use defmt::*;
/// use embassy_executor::Spawner;
/// use embassy_ledeffects::effect::comets;
///
/// #[embassy_executor::main]
/// async fn main(spawner: Spawner) {
///     spawner.spawn(unwrap!(comets::launcher_task(500, 1500)));
/// }
/// ```
/// Signal for a Comet launch every 0.5 to 1.5 seconds.
#[embassy_executor::task]
pub async fn launcher_task(min_delay_ms: Option<u32>, max_delay_ms: Option<u32>) {
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
        let delay = min_delay + (RoscRng.next_u32() % (max_delay - min_delay));
        match with_timeout(Duration::from_millis(delay as u64), IN_MSG.wait()).await {
            Ok(msg) => {
                if let TaskMsg::Stop = msg {
                    break;
                }
            }
            Err(TimeoutError) => LAUNCH.signal(()),
        }
    }
    OUT_MSG.signal(TaskMsg::Ended);
}

/// Stops the comet launcher task by sending it a Stop signal. Then awaits for the task to signal it Ended.
///
/// Returns `Ok(())` if the launcher task sucessfully ended.
/// Returns `Err(embassy_time::TimeoutError)` if the launcher task doesn't respond to the Stop signal in time.
///
/// # Errors
/// Returns [`embassy_time::TimeoutError`] if the comet launcher task doesn't signal it's Ended within 100ms.
pub async fn stop_launcher_task() -> Result<(), TimeoutError> {
    IN_MSG.signal(TaskMsg::Stop);
    // End the loop if timeout expires or get the expected TaskMsg::Ended
    let timeout = Duration::from_millis(100);
    while let Ok(msg) = with_timeout(timeout, OUT_MSG.wait()).await {
        if let TaskMsg::Ended = msg {
            return Ok(());
        }
    }
    Err(TimeoutError)
}

/// Check if the comet launcher task has signaled a Launch.
///
/// Returns `true` (afer awaiting the signal) if a new Launch signal has been sent by the launcher task.
/// Returns `false` immediately if the launcher task has not signaled a Launch.
pub async fn launch_signaled() -> bool {
    if LAUNCH.signaled() {
        LAUNCH.wait().await;
        return true;
    }
    false
}
