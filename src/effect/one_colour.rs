//! The one colour effect sets all the LEDs to a set colour.
//!
//! Can use [smart_leds::colors::WHITE] and [smart_leds::colors::BLACK] to turn on and off the LEDs.
//!
//! # Example
//! Cycle through all the web colours.
//!
//! Refer: `examples/one_colour.rs`
use crate::Strip;
use crate::effect::EffectIterator;
use smart_leds::RGB8;

/// OneColour effect - all LEDs the same colour.
pub struct OneColour {
    colour: RGB8,
    changed: bool,
}

impl OneColour {
    /// Create a new OneColour effect with the specified colour.
    pub fn new(colour: RGB8) -> Self {
        Self {
            colour,
            changed: true,
        }
    }
    /// Set the colour of the effect. This will be applied on the next frame.
    pub fn set(&mut self, colour: RGB8) {
        self.colour = colour;
        self.changed = true;
    }
    /// Get the current colour of the effect.
    pub fn get(&mut self) -> RGB8 {
        self.colour
    }
    /// Force the effect to update the LEDs on the next frame.
    pub fn refresh(&mut self) {
        self.changed = true;
    }
}

impl EffectIterator for OneColour {
    /// Generate the next frame of the one colour effect.
    ///
    /// Does not update the [crate::Strip] if the colour has not changed, unless [OneColour::refresh] has been called.
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()> {
        if self.changed {
            for i in 0..N {
                strip.leds[i] = self.colour;
            }
            self.changed = false;
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
