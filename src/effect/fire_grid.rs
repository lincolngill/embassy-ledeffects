//! Fire effect for a 2D grid of LEDs.
//!
//! The grid can be either vertical or horizontal, and the fire effect is applied to each column respectively.
use crate::Strip;
use crate::effect::EffectIterator;
use crate::effect::fire;
use defmt::Formatter;
use smart_leds::colors;

/// The FireGrid struct holds a 2D array of heat values, the heating and cooling parameters, and the orientation of the grid relative to the [crate::Strip].
pub struct FireGrid<const C: usize, const R: usize> {
    cooling: u8,
    pub sparking: u8,
    heat: [[u8; R]; C],
    pub grid_direction: GridDirection,
}

///The 2D [FireGrid] orientation with regard to the segments of the [crate::Strip].
pub enum GridDirection {
    /// The [crate::Strip] segments are vertical. Each segment being a column.
    Vertical,
    /// The [crate::Strip] segments are horizontal. Each segment being a row.
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
    /// Creates a new FireGrid effect with the given cooling and sparking values.
    ///
    /// The size of the FireGrid heat array is determined by the generic parameters C (columns) and R (rows).
    /// The total number of heat values (C * R) must not exceed the S, length of the [crate::Strip].
    pub fn new<const S: usize>(
        strip: &Strip<S>,
        cooling: Option<u8>,
        sparking: Option<u8>,
        grid_direction: GridDirection,
    ) -> Self {
        assert!(
            C * R <= S,
            "FireGrid<{} x {}> size cannot be > than Strip<{}>",
            C,
            R,
            S
        );
        match grid_direction {
            GridDirection::Vertical => assert!(
                R <= strip.seg_length,
                "R: {} cannot be > strip segment length: {}",
                R,
                strip.seg_length
            ),
            GridDirection::Horizontal => assert!(
                C <= strip.seg_length,
                "C: {} cannot be > strip segment length: {}",
                C,
                strip.seg_length
            ),
        }
        Self {
            cooling: fire::cooling_val(cooling.unwrap_or(DEF_COOLING) as f32, R as f32),
            sparking: sparking.unwrap_or(DEF_SPARKING),
            heat: [[0; R]; C],
            grid_direction,
        }
    }
    /// Increases the cooling value by the given amount, up to a maximum of 255.
    ///
    /// Returns the new cooling value.
    pub fn inc_cooling(&mut self, cooldown: u8) -> u8 {
        self.cooling = self.cooling.saturating_add(cooldown);
        self.cooling
    }
    /// Set the cooling amount. Default: 40
    ///
    /// Returns the new cooling value.
    pub fn set_cooling(&mut self, cooling: Option<u8>) -> u8 {
        self.cooling = fire::cooling_val(cooling.unwrap_or(DEF_COOLING) as f32, R as f32);
        self.cooling
    }
}

impl<const C: usize, const R: usize> EffectIterator for FireGrid<C, R> {
    /// Generates the next frame of the FireGrid effect by updating the heat values and mapping them to colours on the [crate::Strip].
    ///
    /// Uses the [crate::effect::fire] module helper functions to update heat values and map them to colours.
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
                // todo: Handle Layout::Continuous.
                // Currently only handles Layout::ZigZag
                GridDirection::Vertical => {
                    //debug!("i: {} c: {} r: {}", i, c, r);
                    if (c % 2) == 0 {
                        // row inceasing
                        r += 1;
                        if r == strip.seg_length {
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
                        if c == strip.seg_length {
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
