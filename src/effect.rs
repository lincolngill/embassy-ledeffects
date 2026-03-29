//! Selection of LED effects.
//!
//! Each effect is also a crate feature to help minimise the executable size.
#[cfg(feature = "random")]
pub mod random;
use crate::Strip;
#[cfg(feature = "random")]
pub use random::Random;
#[cfg(feature = "wheel")]
pub mod wheel;
#[cfg(feature = "wheel")]
pub use wheel::Wheel;
#[cfg(feature = "onecolour")]
pub mod one_colour;
#[cfg(feature = "onecolour")]
pub use one_colour::OneColour;
#[cfg(feature = "fire")]
pub mod fire;
#[cfg(feature = "fire")]
pub use fire::Fire;
#[cfg(feature = "firegrid")]
pub mod fire_grid;
#[cfg(feature = "firegrid")]
pub use fire_grid::{FireGrid, GridDirection};
#[cfg(feature = "colours")]
mod colours;
#[cfg(feature = "colours")]
use colours::COLOURS;
#[cfg(feature = "comets")]
pub mod comets;
#[cfg(feature = "comets")]
pub use comets::{CometDirection, Comets};

/// Trait for generating the next frame of updates on the Strip.
pub trait EffectIterator {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()>;
}
