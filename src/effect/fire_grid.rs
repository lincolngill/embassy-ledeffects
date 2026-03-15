use crate::Strip;
use crate::effect::EffectIterator;
use crate::effect::fire;
use defmt::Formatter;
use smart_leds::colors;

pub struct FireGrid<const C: usize, const R: usize> {
    cooling: u8,
    pub sparking: u8,
    heat: [[u8; R]; C],
    pub grid_direction: GridDirection,
}

pub enum GridDirection {
    Vertical,
    Horizontal,
}

impl defmt::Format for GridDirection {
    fn format(&self, fmt: Formatter) {
        match self {
            GridDirection::Vertical => defmt::write!(fmt, "Vertical"),
            GridDirection::Horizontal => defmt::write!(fmt, "Horizontal"),
        }
    }
}

const DEF_COOLING: u8 = 40;
const DEF_SPARKING: u8 = 120;

impl<const C: usize, const R: usize> FireGrid<C, R> {
    pub fn new<const S: usize>(
        _: &Strip<S>,
        cooling: Option<u8>,
        sparking: Option<u8>,
        grid_direction: GridDirection,
    ) -> Self {
        if C * R > S {
            panic!(
                "FireGrid<{} x {}> size cannot be greater than Strip<{}>",
                C, R, S
            );
        }
        Self {
            cooling: fire::cooling_val(cooling.unwrap_or(DEF_COOLING) as f32, R as f32),
            sparking: sparking.unwrap_or(DEF_SPARKING),
            heat: [[0; R]; C],
            grid_direction,
        }
    }
    pub fn inc_cooling(&mut self, cooldown: u8) -> u8 {
        self.cooling = self.cooling.saturating_add(cooldown);
        self.cooling
    }
    pub fn set_cooling(&mut self, cooling: Option<u8>) -> u8 {
        self.cooling = fire::cooling_val(cooling.unwrap_or(DEF_COOLING) as f32, R as f32);
        self.cooling
    }
}

impl<const C: usize, const R: usize> EffectIterator for FireGrid<C, R> {
    fn nextframe<const S: usize>(&mut self, strip: &mut Strip<S>) -> Option<()> {
        for c in 0..C {
            fire::update_heat(&mut self.heat[c], self.cooling, self.sparking);
        }
        let mut c = 0;
        let mut r = 0;
        for i in 0..S {
            if c >= C || r >= R {
                strip.leds[i] = colors::BLACK;
            } else {
                strip.leds[i] = fire::colour(self.heat[c][r]);
            }
            match self.grid_direction {
                GridDirection::Vertical => {
                    //debug!("i: {} c: {} r: {}", i, c, r);
                    if (c % 2) == 0 {
                        // row inceasing
                        r += 1;
                        if r == R {
                            c += 1;
                            r -= 1;
                        }
                    } else {
                        // row decreasing
                        if r == 0 {
                            c += 1;
                        } else {
                            r -= 1;
                        }
                    }
                }
                GridDirection::Horizontal => {
                    if (r % 2) == 0 {
                        // col increasing
                        c += 1;
                        if c == C {
                            r += 1;
                            c -= 1;
                        }
                    } else {
                        // col decreassing
                        if c == 0 {
                            r += 1;
                        } else {
                            c -= 1;
                        }
                    }
                }
            }
        }
        strip.inc_frame_cnt();
        Some(())
    }
}
