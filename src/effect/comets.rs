use crate::Strip;
use crate::effect::EffectIterator;
use defmt::Formatter;
use embassy_rp::clocks::RoscRng;
use heapless::spsc::Queue;
use smart_leds::{RGB8, colors};

const MAX_NUM_COMETS: usize = 16;
const START_COOLING_MASK: u8 = 0xFF;

type HeadPos = usize;

#[derive(Copy, Clone)]
pub enum CometDirection {
    Up,
    Down,
}

impl defmt::Format for CometDirection {
    fn format(&self, fmt: Formatter) {
        match self {
            CometDirection::Up => defmt::write!(fmt, "Up"),
            CometDirection::Down => defmt::write!(fmt, "Down"),
        }
    }
}

pub struct Comet {
    next_head_pos: HeadPos,
    direction: CometDirection,
    ttl_pings: u8,
    alive: bool,
}

impl Comet {
    fn new(direction: CometDirection, ttl_pings: u8, strip_len: usize) -> Self {
        Comet {
            next_head_pos: match direction {
                CometDirection::Up => 0,
                CometDirection::Down => strip_len - 1,
            },
            direction,
            ttl_pings,
            alive: true,
        }
    }
    pub fn get_direction(self) -> CometDirection {
        self.direction
    }
    pub fn get_ttl_pings(self) -> u8 {
        self.ttl_pings
    }
}

pub struct Comets<const N: usize> {
    comets: Queue<Comet, MAX_NUM_COMETS>,
    cooling_mask: [u8; N], // todo: rip out the cooling array. Looks better with 0xFF and some random non update frames.
}

impl<const N: usize> Comets<N> {
    pub fn new<const S: usize>(_: &Strip<S>) -> Self {
        assert!(N == S, "Comets<{}> must be same size as Strip<{}>", N, S);
        Self {
            comets: Queue::new(),
            cooling_mask: [0xFF; N],
        }
    }
    pub fn launch(
        &mut self,
        direction: Option<CometDirection>,
        ttl_pings: Option<u8>,
    ) -> Result<(), Comet> {
        const DEF_DIRECTION: CometDirection = CometDirection::Up;
        const DEF_TTL_PINGS: u8 = 0;
        self.comets.enqueue(Comet::new(
            direction.unwrap_or(DEF_DIRECTION),
            ttl_pings.unwrap_or(DEF_TTL_PINGS),
            N,
        ))
    }
    pub fn comet_cnt(&mut self) -> usize {
        self.comets.len()
    }
}

impl<const N: usize> EffectIterator for Comets<N> {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        // cooling
        for i in 0..S {
            if strip.leds[i] != colors::BLACK {
                if RoscRng.next_u32() % 3 == 0 {
                    let rn = RoscRng.next_u32();
                    strip.leds[i] = RGB8 {
                        r: strip.leds[i]
                            .r
                            .saturating_sub(rn as u8 & self.cooling_mask[i]),
                        g: strip.leds[i]
                            .g
                            .saturating_sub((rn >> 8) as u8 & self.cooling_mask[i]),
                        b: strip.leds[i]
                            .b
                            .saturating_sub((rn >> 16) as u8 & self.cooling_mask[i]),
                    };
                }
            }
            // Increase max cooling, for pixels not there yet.
            if self.cooling_mask[i] < 0xFF {
                self.cooling_mask[i] = self.cooling_mask[i] << 1 | 1;
            }
        }
        // Update next_head_pos
        for c in self.comets.iter_mut() {
            if c.alive {
                strip.leds[c.next_head_pos] = colors::WHITE;
                // Sets lower pixel cooling near head
                self.cooling_mask[c.next_head_pos] = START_COOLING_MASK;
                match c.direction {
                    CometDirection::Up => {
                        c.next_head_pos += 1;
                        if c.next_head_pos == S {
                            if c.ttl_pings == 0 {
                                c.alive = false;
                            } else {
                                c.ttl_pings -= 1;
                                c.direction = CometDirection::Down;
                                c.next_head_pos -= 1;
                            }
                        }
                    }
                    CometDirection::Down => {
                        if c.next_head_pos == 0 {
                            if c.ttl_pings == 0 {
                                c.alive = false;
                            } else {
                                c.ttl_pings -= 1;
                                c.direction = CometDirection::Up;
                            }
                        } else {
                            c.next_head_pos -= 1;
                        }
                    }
                }
            }
        }
        // Deq oldest comet if it's dead
        if let Some(c) = self.comets.peek()
            && !c.alive
        {
            self.comets.dequeue().unwrap();
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
