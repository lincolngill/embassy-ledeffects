//! The random effect changes each LED at random times to a random colour.
//!
//! The per LED random change period is between ~(500 - 2540 ms).
//! Colours are randomly selected from [crate::effect::COLOURS].
//!
//! # Example
//!
//! Refer to `examples/random.rs` for a complete example of the random effect in use.
//!
//! Refer to `examples/panel_buttons.rs` and `examples/strip_buttons.rs` for examples of using the random effect with buttons to change the delay factor.
use crate::Strip;
use crate::effect::{COLOURS, EffectIterator};
use embassy_rp::clocks::RoscRng;

const MAX_DELAY_FACTOR: u64 = 512;
const MIN_DELAY_FACTOR: u64 = 8;

/// The Random effect struct holds a separate timeout value for each LED.
pub struct Random<const N: usize> {
    led_timeout: [u64; N],
    delay_factor: u64,
    colours_cnt: usize,
}

impl<const N: usize> Random<N> {
    /// Create a new Random colour effect with the specified delay factor.
    ///
    /// The delay factor is used to adjust the random delay period for each LED. The random delay period is calculated as follows:
    /// ```rust
    /// let rn = RoscRng.next_u32();
    /// self.led_timeout[i] = now + 500 + ((rn >> 24) as u64 * self.delay_factor);
    /// ```
    /// 500 ms + ( an 8bit random number * the delay factor )
    pub fn new<const S: usize>(_: &Strip<S>, delay_factor: Option<u64>) -> Self {
        let df = delay_factor.unwrap_or(MIN_DELAY_FACTOR);
        assert!(
            df >= MIN_DELAY_FACTOR && df <= MAX_DELAY_FACTOR,
            "delay_factor must be between {} and {}",
            MIN_DELAY_FACTOR,
            MAX_DELAY_FACTOR
        );
        // Use size of Strip to make sure Random is the same size.
        assert!(N == S, "Random<{}> must be same size as Strip<{}>", N, S);
        Self {
            led_timeout: [0; N],
            delay_factor: df,
            colours_cnt: COLOURS.len(),
        }
    }
    /// Slow down the effect by increasing the delay factor.
    ///
    /// Doubles the delay factor up to a maxium of 512. Wraps back to the minimum of 8 if the maximum is exceeded.
    pub fn slow_down(&mut self) -> u64 {
        self.delay_factor *= 2;
        if self.delay_factor > MAX_DELAY_FACTOR {
            self.delay_factor = MIN_DELAY_FACTOR
        }
        let now = embassy_time::Instant::now().as_millis();
        for i in 0..N {
            let rn = RoscRng.next_u32();
            self.led_timeout[i] = now + 500 + ((rn >> 24) as u64 * self.delay_factor);
        }
        self.delay_factor
    }
}

impl<const N: usize> EffectIterator for Random<N> {
    /// Generate the next frame of the random colour effect.
    ///
    /// The timeout for each LED is checked. If it has expired the LED is changed to a new random colour and a new timeout is set.
    /// Refer to [Random::new] for the timeout calculation.
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        let now = embassy_time::Instant::now().as_millis();
        for i in 0..N {
            if self.led_timeout[i] < now {
                let rn = RoscRng.next_u32();
                let ci = rn as usize % self.colours_cnt;
                strip.leds[i] = COLOURS[ci].colour;
                /*
                strip.leds[i] = RGB8 {
                    r: (rn & 0xFF) as u8,
                    g: ((rn >> 8) & 0xFF) as u8,
                    b: ((rn >> 16) & 0xFF) as u8,
                };
                */
                self.led_timeout[i] = now + 500 + ((rn >> 24) as u64 * self.delay_factor);
            }
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
