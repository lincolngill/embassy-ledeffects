mod random;
use crate::Strip;
pub use random::Random;
mod wheel;
pub use wheel::Wheel;
mod off;
pub use off::Off;

// Triat for generating the next frame of updates on the Strip.
pub trait EffectIterator {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()>;
}
