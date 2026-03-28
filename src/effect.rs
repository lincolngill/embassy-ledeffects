#[cfg(feature = "random")]
mod random;
use crate::Strip;
#[cfg(feature = "random")]
pub use random::Random;
#[cfg(feature = "wheel")]
mod wheel;
#[cfg(feature = "wheel")]
pub use wheel::Wheel;
#[cfg(feature = "onecolour")]
mod one_colour;
#[cfg(feature = "onecolour")]
pub use one_colour::OneColour;
#[cfg(feature = "fire")]
mod fire;
#[cfg(feature = "fire")]
pub use fire::Fire;
#[cfg(feature = "firegrid")]
mod fire_grid;
#[cfg(feature = "firegrid")]
pub use fire_grid::{FireGrid, GridDirection};
#[cfg(feature = "colours")]
mod colours;
#[cfg(feature = "colours")]
pub use colours::COLOURS;
#[cfg(feature = "comets")]
pub mod comets;
#[cfg(feature = "comets")]
pub use comets::{CometDirection, Comets};

// Triat for generating the next frame of updates on the Strip.
pub trait EffectIterator {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()>;
}
