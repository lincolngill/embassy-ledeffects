mod random;
use crate::Strip;
pub use random::Random;

pub trait EffectIterator {
    fn nextframe<const N: usize>(&mut self, strip: &mut Strip<N>) -> Option<()>;
}
