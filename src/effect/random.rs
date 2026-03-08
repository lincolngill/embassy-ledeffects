use crate::Strip;
use crate::effect::EffectIterator;
use crate::strip::NUM_LEDS;
use embassy_rp::clocks::RoscRng;
use smart_leds::RGB8;

pub struct Random {
    led_timeout: [u64; NUM_LEDS],
    rng: RoscRng,
}

impl Random {
    pub fn new() -> Self {
        Self {
            led_timeout: [0; NUM_LEDS],
            rng: RoscRng,
        }
    }
}

impl EffectIterator for Random {
    fn nextframe(&mut self, strip: &mut Strip) -> Option<()> {
        let now = embassy_time::Instant::now().as_millis();
        for i in 0..NUM_LEDS {
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
