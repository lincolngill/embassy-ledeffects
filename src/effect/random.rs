use crate::Strip;
use crate::effect::EffectIterator;
use embassy_rp::clocks::RoscRng;
use smart_leds::RGB8;

pub struct Random<const N: usize> {
    led_timeout: [u64; N],
    rng: RoscRng,
}

impl<const N: usize> Random<N> {
    pub fn new() -> Self {
        Self {
            led_timeout: [0; N],
            rng: RoscRng,
        }
    }
}

impl<const N: usize> EffectIterator for Random<N> {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        let now = embassy_time::Instant::now().as_millis();
        for i in 0..S {
            if self.led_timeout[i] < now {
                let rn = self.rng.next_u32();
                strip.leds[i] = RGB8 {
                    r: (rn & 0xFF) as u8,
                    g: ((rn >> 8) & 0xFF) as u8,
                    b: ((rn >> 16) & 0xFF) as u8,
                };
                self.led_timeout[i] = now + 500 + ((rn >> 24) as u64 * 8);
            }
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
