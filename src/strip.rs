/*
LED strip of a given size.

Note: Can only have one strip. Uses global static variables to control FPS.

A 2D panel of LEDs is represented by a strip of LEDS, with a segment length and layout.
Segment length = number of LEDs in a row.
Layout = Either ZipZag or Continuous. ZigZag is normally how a 2D panel is arranged.

E.g. a 5 col x 3 rows LED strip with ZipZag layout would be a:
15 LED strip, with;
Segment legnth of 5, and;
LEDs numbered as follows:

10 11 12 13 14
 9  8  7  6  5
 0  1  2  3  4

*/
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;

pub enum Layout {
    ZigZag,
    Continuous,
}

pub struct Strip<const N: usize> {
    pub leds: [RGB8; N],
    pub seg_length: usize,
    pub layout: Layout,
}

impl<const N: usize> Strip<N> {
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
    pub fn inc_frame_cnt(&self) {
        FRAME_CNT.fetch_add(1, Ordering::Relaxed);
    }
    pub fn frame_delay(&self) -> Duration {
        Duration::from_millis(FRAME_DELAY_MS.load(Ordering::Relaxed) as u64)
    }
}

pub static FRAME_CNT: AtomicU32 = AtomicU32::new(0);
pub static FRAME_DELAY_MS: AtomicU32 = AtomicU32::new(100);

// Task to adjust FRAME_DELAY_MS to obtain FPS target.
// Make adjustments every fps_adjust_secs seconds.
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
        let delta_fps = fps - fps_target;
        let mut delta_delay: i32 = 0;
        if fps != 0 {
            delta_delay = fpst_delay - 1000 / fps;
        }
        let new_delay = (delay + delta_delay).max(MIN_DELAY_MS);
        debug!(
            "FPS: {} Delta FPS: {} Delta Delay: {} Delay: {} NewDelay: {} Frames: {}",
            fps, delta_fps, delta_delay, delay, new_delay, end_cnt
        );
        if new_delay != delay {
            FRAME_DELAY_MS.store(new_delay as u32, Ordering::Relaxed);
            delay = new_delay;
        }
    }
}
