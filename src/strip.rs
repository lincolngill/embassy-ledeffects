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
use core::cmp;
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
                fps_tolerance = fps_diff.abs().min(3);
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

/* Something not quirte right!
280.077518 [DEBUG] FPS: 41 FPS_Diff: -19 FPS_Tolerance: 0 Delay: 1 NewDelay: 0 Frames: 15837 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
285.078043 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 16055 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
290.078549 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 16273 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
295.082228 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 16491 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
300.085117 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 16709 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
305.089888 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 16927 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
310.093946 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 17145 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
315.098910 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 17363 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
316.476128 [DEBUG] btn2 Fire (effect_buttons src/bin/effect_buttons.rs:290)
320.104448 [DEBUG] FPS: 43 FPS_Diff: -17 FPS_Tolerance: 3 Delay: 0 NewDelay: 0 Frames: 17581 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
320.334136 [DEBUG] btn2 Fire (effect_buttons src/bin/effect_buttons.rs:290)
323.877899 [INFO ] EffectState: Random (effect_buttons src/bin/effect_buttons.rs:298)
325.104990 [DEBUG] FPS: 61 FPS_Diff: 1 FPS_Tolerance: 3 Delay: 0 NewDelay: 1 Frames: 17887 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
330.105507 [DEBUG] FPS: 115 FPS_Diff: 55 FPS_Tolerance: 0 Delay: 0 NewDelay: 1 Frames: 18466 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
335.106000 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 18986 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
340.106499 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 19506 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
345.106957 [DEBUG] FPS: 103 FPS_Diff: 43 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 20025 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
350.107452 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 20545 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
355.107951 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 21065 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
360.108464 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 21585 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
365.108961 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 22105 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
370.110363 [DEBUG] FPS: 104 FPS_Diff: 44 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 22625 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
371.649535 [INFO ] EffectState: Wheel (effect_buttons src/bin/effect_buttons.rs:298)
375.110893 [DEBUG] FPS: 101 FPS_Diff: 41 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 23134 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
380.111500 [DEBUG] FPS: 101 FPS_Diff: 41 FPS_Tolerance: 3 Delay: 1 NewDelay: 1 Frames: 23640 (embassy_ledeffects embassy-ledeffects/src/strip.rs:100)
*/
