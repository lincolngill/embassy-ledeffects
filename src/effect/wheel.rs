//! The Wheel effect cycles through a rainbow of colours
use crate::Strip;
use crate::effect::EffectIterator;
use smart_leds::RGB8;

pub const MAX_SPEED: u16 = 10;

/// The Wheel strut contains the speed of the effect and the current position within the rainbow cycle.
pub struct Wheel {
    pos: u16,
    speed: u16,
}

impl Wheel {
    /// Creates a new Wheel effect with the given speed.
    ///
    /// The speed determines how fast the colours cycle through the rainbow. Default is 1, which means the colours will cycle every 256 frames. Higher values will cycle faster, with a maximum of [`MAX_SPEED`].
    pub fn new(speed: Option<u16>) -> Self {
        let s = speed.unwrap_or(1);
        assert!(s > 0, "Speed must be between 1 and {}", MAX_SPEED);
        Self { pos: 0, speed: s }
    }
    /// Increments the speed of the effect. If the speed value exceeds [`MAX_SPEED`], it wraps back to 1.
    pub fn speedup(&mut self) -> u16 {
        self.speed += 1;
        if self.speed > MAX_SPEED {
            self.speed = 1;
        }
        self.speed
    }
}

///A helper function to generate a colour from a position in the rainbow cycle.
///
/// The colours transition from read to green to blue, then back to read.
///
/// # Arguments
/// * `wheel_pos` - A value from 0 to 255 that represents a position in the rainbow cycle.
///
/// Returns the RGB8 colour that corresponds to that position in the cycle.
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
    /// Generates the next frame of the Wheel effect.
    ///
    /// The LED colours are updated based on their position in the strip and the current position in the rainbow cycle.
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
