use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_time::{Duration, Timer};
use smart_leds::RGB8;

pub const NUM_LEDS: usize = 120;
pub struct Strip {
    pub leds: [RGB8; NUM_LEDS],
}

impl Strip {
    pub fn new(target_fps: u32) -> Self {
        FRAME_DELAY_MS.store(1000 / target_fps, Ordering::Relaxed);
        Self {
            leds: [RGB8::default(); NUM_LEDS],
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

#[embassy_executor::task]
pub async fn frame_rate_task(refresh_secs: u32, target_fps: u32) {
    let mut tolerance = 0;
    loop {
        let start_cnt = FRAME_CNT.load(Ordering::Relaxed);
        Timer::after_secs(refresh_secs as u64).await;
        let end_cnt = FRAME_CNT.load(Ordering::Relaxed);
        let fps = (end_cnt - start_cnt) / refresh_secs;
        let fps_diff = fps as i32 - target_fps as i32;
        let delay = FRAME_DELAY_MS.load(Ordering::Relaxed);
        let mut new_delay = delay;
        if fps_diff.abs() > tolerance {
            new_delay = delay * fps / target_fps;
            if new_delay == delay {
                tolerance = fps_diff.abs() 
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
