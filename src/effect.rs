mod random;
use crate::Strip;
pub use random::Random;

pub trait EffectIterator {
    fn nextframe(&mut self, strip: &mut Strip) -> Option<()>;
}
