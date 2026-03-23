use crate::Strip;
use crate::effect::EffectIterator;
use smart_leds::RGB8;

pub struct OneColour {
    colour: RGB8,
    changed: bool,
}

impl OneColour {
    pub fn new(colour: RGB8) -> Self {
        Self {
            colour,
            changed: true,
        }
    }
    pub fn set(&mut self, colour: RGB8) {
        self.colour = colour;
        self.changed = true;
    }
    pub fn get(&mut self) -> RGB8 {
        self.colour
    }
    pub fn refresh(&mut self) {
        self.changed = true;
    }
}

impl EffectIterator for OneColour {
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
