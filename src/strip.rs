/*
LED strip of a given size.

Note: Can only have one strip. Uses global static variables to control FPS.
*/
use core::cmp;
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;

pub struct Strip<const N: usize> {
    pub leds: [RGB8; N],
}

impl<const N: usize> Strip<N> {
    pub fn new() -> Self {
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
pub static FRAME_DELAY_MS: AtomicU32 = AtomicU32::new(100);

// Task to adjust FRAME_DELAY_MS to obtain FPS target.
// Make adjustments every fps_adjust_secs seconds.
#[embassy_executor::task]
pub async fn frame_rate_task(fps_adjust_secs: u32, fps_target: u32) {
    defmt::assert!(fps_adjust_secs > 0, "FPS adjustment secs must be > 0");
    defmt::assert!(fps_target > 0, "FPS target must be > 0");
    let mut fps_tolerance = 0;
    let mut delay = 1000 / fps_target;
    FRAME_DELAY_MS.store(delay, Ordering::Relaxed);
    loop {
        let start_cnt = FRAME_CNT.load(Ordering::Relaxed);
        Timer::after_secs(fps_adjust_secs as u64).await;
        let end_cnt = FRAME_CNT.load(Ordering::Relaxed);
        let fps = (end_cnt - start_cnt) / fps_adjust_secs;
        let fps_diff = fps as i32 - fps_target as i32;
        let current_delay = delay;
        let new_delay = cmp::max(delay, 1) * fps / fps_target;
        if fps > 0 && fps_diff.abs() > fps_tolerance {
            if new_delay == delay {
                fps_tolerance = fps_diff.abs()
                // Could break out of loop and end task here.
                // Or check for no adjustment for n iterations and then exit.
                // May implement this if all future effects have a stable delay value.
            } else {
                delay = new_delay;
                FRAME_DELAY_MS.store(new_delay, Ordering::Relaxed);
                fps_tolerance = 0;
            }
        }
        debug!(
            "FPS: {} FPS_Diff: {} FPS_Tolerance: {} Delay: {} NewDelay: {} Frames: {}",
            fps, fps_diff, fps_tolerance, current_delay, new_delay, end_cnt
        );
    }
}
