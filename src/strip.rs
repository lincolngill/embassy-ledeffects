//! LED strip of a given size.
//!
//! Note: You can only have one Strip. Global static variables are used to control the FPS.
//!
//! A 2D panel of LEDs is represented by a strip of LEDS, with a segment length and layout.
//! * Segment length = number of LEDs in a row.
//! * Layout = Either ZipZag or Continuous. ZigZag is normally how a 2D panel is arranged.
//!
//! E.g. a 5 col x 3 rows LED strip with ZipZag layout would be a:
//!
//! 15 LED strip, with:
//! * Segment legnth of 5, and;
//! * LEDs numbered as follows:
//!
//! ```
//! 10 11 12 13 14
//!  9  8  7  6  5
//!  0  1  2  3  4
//! ```
//!
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;

/// Layout of the LED strip segments. ZigZag or Continuous.
pub enum Layout {
    /// ZigZag layout is where each segment of the strip is reversed. This is common for 2D panels.
    ///
    /// # Example
    /// LEDs: 15, Segment Size: 5, Layout:ZigZag
    /// ```
    /// 10 11 12 13 14
    ///  9  8  7  6  5
    ///  0  1  2  3  4
    /// ```
    /// Row: 3 Cols: 5
    ZigZag,
    /// Continuous layout is where each segment of the strip is in the same direction.
    /// This allows a single Strip to wrap around a cylinder and be treated as a 2D panel of rows and columns.
    ///
    /// # Example
    /// LEDs: 15, Segment Size: 5, Layout:Continuous
    /// ```
    /// 10 11 12 13 14
    ///  5  6  7  8  9
    ///  0  1  2  3  4
    /// ```
    /// Row: 3 Cols: 5
    Continuous,
}

/// The Strip struct represents an LED strip of a given size, with a segment length and layout.
///
/// The segment length alllows the single Strip to repressent a 2D panel of LEDs.
///
/// N is the total number of LEDs.
pub struct Strip<const N: usize> {
    pub leds: [RGB8; N],
    pub seg_length: usize,
    pub layout: Layout,
}

impl<const N: usize> Strip<N> {
    /// Create a new LED Strip.
    ///
    /// # Example
    /// ```
    /// let strip = Strip::<15>::new(Some(5), Some(Layout::ZigZag));
    /// ```
    pub fn new(segment_length: Option<usize>, segment_layout: Option<Layout>) -> Self {
        let seg_length = segment_length.unwrap_or(N);
        defmt::assert!(seg_length <= N, "Segment length must be less than {}", N);
        defmt::assert!(
            N > 0 && seg_length > 0,
            "N: {} and segment length: {}, must be > 0",
            N,
            seg_length
        );
        let layout = segment_layout.unwrap_or({
            if seg_length < N {
                Layout::ZigZag
            } else {
                Layout::Continuous
            }
        });
        Self {
            leds: [RGB8::default(); N],
            seg_length,
            layout,
        }
    }
    /// Increment the frame count by 1.
    ///
    /// The frame count is used by the [frame_rate_task] to adjust the frame delay to meet the FPS target.
    /// Each effect's implementation of [crate::effect::EffectIterator] calls this method once in it's nextframe() implementation.
    /// The frame count is used to calculate the current FPS.
    pub fn inc_frame_cnt(&self) {
        FRAME_CNT.fetch_add(1, Ordering::Relaxed);
    }
    /// Get the current frame delay loop duration.
    ///
    /// The frame delay is adjusted by the [frame_rate_task] to meet the FPS target.
    ///
    /// # Returns
    /// The Duration that should be used in the main loop to achieve the target FPS.
    pub fn frame_delay(&self) -> Duration {
        Duration::from_millis(FRAME_DELAY_MS.load(Ordering::Relaxed) as u64)
    }
}

/// Global static atomic frame counter variable.
///
/// Effects call [Strip::inc_frame_cnt] to update the frame count.
/// The [frame_rate_task] reads this counter to calculate the current FPS.
static FRAME_CNT: AtomicU32 = AtomicU32::new(0);
/// Global static atomic frame delay variable in milliseconds.
///
/// The [frame_rate_task] adjust the frame delay value to meet the target FPS.
/// The main loop should read and use this value as the per frame sleep delay. This should achieve the target FPS.
static FRAME_DELAY_MS: AtomicU32 = AtomicU32::new(100);

/// Embassy task to calculate the required main loop delay to achieve the target FPS.
///
/// Updates the delay between frames periodically to achieve the target FPS. Refer: [Strip::frame_delay].
///
/// # Arguments
/// * `fps_adjust_secs` - The number of seconds between each FPS adjustment. Recommend: 5 seconds.
/// * `fps_target` - The target FPS. Recommend: 30-60 FPS.
///
/// # Example
/// ```rust
/// use embassy_executor::Spawner;
/// use embassy_ledeffects::strip;
///
/// const FPS_TARGET: i32 = 60;
/// const FPS_ADJUST_SECS: i32 = 5;
///
/// #[embassy_executor::main]
/// async fn main(spawner: Spawner) {
///     spawner.spawn(unwrap!(strip::frame_rate_task(FPS_ADJUST_SECS, FPS_TARGET)));
/// }
/// ```
#[embassy_executor::task]
pub async fn frame_rate_task(fps_adjust_secs: i32, fps_target: i32) {
    const MIN_DELAY_MS: i32 = 2;
    defmt::assert!(fps_adjust_secs > 0, "FPS adjustment secs must be > 0");
    defmt::assert!(fps_target > 0, "FPS target must be > 0");
    let fpst_delay = 1000 / fps_target;
    let mut delay = fpst_delay;
    FRAME_DELAY_MS.store(fpst_delay as u32, Ordering::Relaxed);
    loop {
        let start_cnt = FRAME_CNT.load(Ordering::Relaxed);
        Timer::after_secs(fps_adjust_secs as u64).await;
        let end_cnt = FRAME_CNT.load(Ordering::Relaxed);
        let fps = (end_cnt - start_cnt) as i32 / fps_adjust_secs;
        let delta_fps = fps_target - fps;
        let mut delta_delay: i32 = 0;
        if fps != 0 {
            delta_delay = fpst_delay - 1000 / fps;
        }
        let new_delay = (delay + delta_delay).max(MIN_DELAY_MS);
        if new_delay != delay {
            FRAME_DELAY_MS.store(new_delay as u32, Ordering::Relaxed);
            debug!(
                "FPS: {} Delta FPS: {} Delta Delay: {} Delay: {} NewDelay: {} Frames: {}",
                fps, delta_fps, delta_delay, delay, new_delay, end_cnt
            );
            delay = new_delay;
        } else {
            debug!(
                "FPS: {} Delta FPS: {} Delta Delay: {} Delay: {} (No Change) Frames: {}",
                fps, delta_fps, delta_delay, delay, end_cnt
            );
        }
    }
}
