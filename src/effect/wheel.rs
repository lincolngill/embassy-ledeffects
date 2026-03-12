use crate::Strip;
use crate::effect::EffectIterator;
use smart_leds::RGB8;

const MAX_SPEED: u16 = 10;
pub struct Wheel {
    pos: u16,
    speed: u16,
}

impl Wheel {
    pub fn new(speed: Option<u16>) -> Self {
        let s = speed.unwrap_or(1);
        assert!(s > 0, "Speed must be between 1 and {}", MAX_SPEED);
        Self { pos: 0, speed: s }
    }
    pub fn speedup(&mut self) -> u16 {
        self.speed += 1;
        if self.speed > MAX_SPEED {
            self.speed = 1;
        }
        self.speed
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}

impl EffectIterator for Wheel {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()> {
        for i in 0..N {
            strip.leds[i] = wheel((((i * 256) as u16 / N as u16 + self.pos) & 255) as u8);
        }
        self.pos += self.speed;
        if self.pos >= 256 * 5 {
            self.pos = 0;
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
