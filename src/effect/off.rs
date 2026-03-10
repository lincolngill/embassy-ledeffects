use crate::Strip;
use crate::effect::EffectIterator;
use smart_leds::RGB8;

pub struct Off {
    off_led: RGB8,
}

impl Off {
    pub fn new() -> Self {
        Self {
            off_led: RGB8::default(),
        }
    }
}

impl EffectIterator for Off {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()> {
        for i in 0..N {
            strip.leds[i] = self.off_led;
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
