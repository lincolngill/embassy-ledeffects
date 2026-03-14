mod random;
use crate::Strip;
pub use random::Random;
mod wheel;
pub use wheel::Wheel;
mod one_colour;
pub use one_colour::OneColour;
mod fire;
pub use fire::Fire;
mod fire_grid;
use defmt::Formatter;
pub use fire_grid::{FireGrid, StripDirection};

// Triat for generating the next frame of updates on the Strip.
pub trait EffectIterator {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()>;
}

pub enum Effect {
    Random,
    Wheel,
    OneColour,
    HFireGrid,
    VFireGrid,
    Fire,
}
impl defmt::Format for Effect {
    fn format(&self, fmt: Formatter) {
        match self {
            Effect::Random => defmt::write!(fmt, "Random"),
            Effect::Wheel => defmt::write!(fmt, "Wheel"),
            Effect::OneColour => defmt::write!(fmt, "OneColour"),
            Effect::HFireGrid => defmt::write!(fmt, "HFireGrid"),
            Effect::VFireGrid => defmt::write!(fmt, "VFireGrid"),
            Effect::Fire => defmt::write!(fmt, "Fire"),
        }
    }
}
