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
pub use fire_grid::{FireGrid, GridDirection};
mod colours;
pub use colours::COLOURS;
mod comets;
pub use comets::Comets;

// Triat for generating the next frame of updates on the Strip.
pub trait EffectIterator {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()>;
}
