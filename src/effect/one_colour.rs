use crate::Strip;
use crate::effect::EffectIterator;
use smart_leds::RGB8;

pub struct OneColour {
    pub colour: RGB8,
}

impl OneColour {
    pub fn new(colour: RGB8) -> Self {
        Self { colour }
    }
}

impl EffectIterator for OneColour {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()> {
        for i in 0..N {
            strip.leds[i] = self.colour;
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
