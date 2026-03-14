use crate::Strip;
use crate::effect::EffectIterator;
use embassy_rp::clocks::RoscRng;
use smart_leds::RGB8;

const MAX_DELAY_FACTOR: u64 = 512;
const MIN_DELAY_FACTOR: u64 = 8;

pub struct Random<const N: usize> {
    // Keep a separate timeout value for each pixel.
    led_timeout: [u64; N],
    delay_factor: u64,
}

impl<const N: usize> Random<N> {
    pub fn new<const S: usize>(_: &Strip<S>, delay_factor: Option<u64>) -> Self {
        let df = delay_factor.unwrap_or(MIN_DELAY_FACTOR);
        assert!(
            df >= MIN_DELAY_FACTOR && df <= MAX_DELAY_FACTOR,
            "delay_factor must be between {} and {}",
            MIN_DELAY_FACTOR,
            MAX_DELAY_FACTOR
        );
        // Use size of Strip to make sure Random is the same size.
        if N != S {
            panic!("Random<{}> must be the same size as Strip<{}>", N, S);
        }
        Self {
            led_timeout: [0; N],
            delay_factor: df,
        }
    }
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
    // Random colours
    // Change each pixel at random times. Between 500 - 2540ms.
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        let now = embassy_time::Instant::now().as_millis();
        for i in 0..N {
            if self.led_timeout[i] < now {
                let rn = RoscRng.next_u32();
                strip.leds[i] = RGB8 {
                    r: (rn & 0xFF) as u8,
                    g: ((rn >> 8) & 0xFF) as u8,
                    b: ((rn >> 16) & 0xFF) as u8,
                };
                self.led_timeout[i] = now + 500 + ((rn >> 24) as u64 * self.delay_factor);
            }
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
