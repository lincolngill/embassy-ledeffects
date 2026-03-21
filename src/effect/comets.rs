use crate::Strip;
use crate::effect::EffectIterator;
//use defmt::debug;
use embassy_rp::clocks::RoscRng;
use heapless::spsc::Queue;
use smart_leds::{RGB8, colors};

const MAX_NUM_COMETS: usize = 16;
const COMET_TAIL: usize = 1;

type HeadPos = usize;

pub struct Comets {
    comets: Queue<HeadPos, MAX_NUM_COMETS>,
    //    frames: usize,
}

impl Comets {
    pub fn new() -> Self {
        Self {
            comets: Queue::new(),
            //            frames: 0,
        }
    }
    pub fn launch(&mut self) -> Result<(), HeadPos> {
        self.comets.enqueue(0)
    }
    pub fn comet_cnt(&mut self) -> usize {
        self.comets.len()
    }
}

#[inline]
fn cooldown2(pixel: RGB8) -> RGB8 {
    const MASK: u32 = 0xAF;
    let rn = RoscRng.next_u32();
    RGB8 {
        r: pixel.r.saturating_sub((rn & MASK) as u8),
        g: pixel.g.saturating_sub(((rn >> 8) & MASK) as u8),
        b: pixel.b.saturating_sub(((rn >> 16) & MASK) as u8),
    }
}

impl EffectIterator for Comets {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        let mut ci = self.comets.iter().rev();
        let mut c = ci.next();
        let mut next_head_pos = match c {
            Some(hp) => *hp,
            None => S + COMET_TAIL,
        };
        for i in 0..S {
            if i == next_head_pos {
                strip.leds[i] = colors::WHITE;
                c = ci.next();
                next_head_pos = match c {
                    Some(hp) => *hp,
                    None => S + COMET_TAIL,
                };
                /*
                debug!(
                    "Frame: {} At: {} next_head_pos: {}",
                    self.frames, i, next_head_pos
                );
                */
                continue;
            }
            if strip.leds[i] == colors::BLACK {
                continue;
            }
            //strip.leds[i] = cooldown(strip.leds[i], next_head_pos - i);
            strip.leds[i] = cooldown2(strip.leds[i]);
            /*
            debug!(
                "Frame: {} At: {} Div: {} Cooldown: {} {} {}",
                self.frames, i, cooldown_divisor, strip.leds[i].r, strip.leds[i].g, strip.leds[i].b
            );
            */
        }
        for c in self.comets.iter_mut() {
            *c += 1;
        }
        if let Some(c) = self.comets.peek()
            && *c >= (S + COMET_TAIL)
        {
            self.comets.dequeue().unwrap();
        }
        strip.inc_frame_cnt();
        /*
        if self.comets.len() > 1 {
            self.frames += 1;
            if self.frames > 100 {
                panic!("Early exit");
            }
        };
        */
        Some(())
    }
}
