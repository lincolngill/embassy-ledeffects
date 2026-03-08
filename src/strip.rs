/*
LED strip of a given size.

Note: Can only have one strip. Uses global static variables to control FPS.
*/
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;

pub struct Strip<const N: usize> {
    pub leds: [RGB8; N],
}

impl<const N: usize> Strip<N> {
    pub fn new(fps_target: u32) -> Self {
        FRAME_DELAY_MS.store(1000 / fps_target, Ordering::Relaxed);
        Self {
            leds: [RGB8::default(); N],
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
pub static FRAME_DELAY_MS: AtomicU32 = AtomicU32::new(0);

// Task to adjust FRAME_DELAY_MS to obtain FPS target.
// Make adjustments every fps_adjust_secs seconds.
#[embassy_executor::task]
pub async fn frame_rate_task(fps_adjust_secs: u32, fps_target: u32) {
    let mut tolerance = 0;
    loop {
        let start_cnt = FRAME_CNT.load(Ordering::Relaxed);
        Timer::after_secs(fps_adjust_secs as u64).await;
        let end_cnt = FRAME_CNT.load(Ordering::Relaxed);
        let fps = (end_cnt - start_cnt) / fps_adjust_secs;
        let fps_diff = fps as i32 - fps_target as i32;
        let delay = FRAME_DELAY_MS.load(Ordering::Relaxed);
        let mut new_delay = delay;
        if fps_diff.abs() > tolerance {
            new_delay = delay * fps / fps_target;
            if new_delay == delay {
                tolerance = fps_diff.abs()
                // Could break out of loop and end task here.
                // Or check for no adjustment for n iterations and then exit.
                // May implement this if all future effects have a stable delay value.
            } else {
                FRAME_DELAY_MS.store(new_delay, Ordering::Relaxed);
                tolerance = 0;
            }
        }
        debug!(
            "FPS: {} Delay: {} Diff: {} NewDelay: {} Tolerance: {} Frames: {}",
            fps, delay, fps_diff, new_delay, tolerance, end_cnt
        );
    }
}
