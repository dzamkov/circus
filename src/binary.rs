use crate::*;

/// A system which allows arbitrary boolean operations.
pub trait BinarySystem:
    SystemBitAnd<bool> + SystemBitOr<bool> + SystemBitXor<bool> + SystemNot<bool>
{
}

impl<S: SystemBitAnd<bool> + SystemBitOr<bool> + SystemBitXor<bool> + SystemNot<bool>> BinarySystem
    for S
{
}

/// Augments a [`BinarySystem`] with operations on other types by representing them as strings
/// of binary values.
#[repr(transparent)]
pub struct BinaryEmulate<S>(S);

impl<S: BinarySystem> BinaryEmulate<S> {
    /// Constructs a new [`BinaryEmulate`] wrapper over the given [`BinarySystem`].
    pub fn new(source: S) -> Self {
        Self(source)
    }
}

impl<S: BinarySystem> SystemRepr<bool> for BinaryEmulate<S> {
    type Abstract = Abstract<S, bool>;
    fn constant(&mut self, value: bool) -> Self::Abstract {
        self.0.constant(value)
    }
}

impl<S: BinarySystem> SystemRepr<u32> for BinaryEmulate<S> {
    type Abstract = [Abstract<S, bool>; 32];
    fn constant(&mut self, value: u32) -> Self::Abstract {
        array_init::array_init(|i| self.constant(value >> i != 0))
    }
}

impl<S: BinarySystem> SystemBitAnd<bool> for BinaryEmulate<S> {
    fn and(&mut self, a: &Abstract<Self, bool>, b: &Abstract<Self, bool>) -> Abstract<Self, bool> {
        self.0.and(a, b)
    }
}

impl<S: BinarySystem> SystemBitXor<bool> for BinaryEmulate<S> {
    fn xor(&mut self, a: &Abstract<Self, bool>, b: &Abstract<Self, bool>) -> Abstract<Self, bool> {
        self.0.xor(a, b)
    }
}

impl<S: BinarySystem> SystemWrappingAdd<u32> for BinaryEmulate<S> {
    fn wrapping_add(
        &mut self,
        a: &Abstract<Self, u32>,
        b: &Abstract<Self, u32>,
    ) -> Abstract<Self, u32> {
        todo!()
    }
}

#[test]
fn test_wrapping_add() {
    let mut sys = BinaryEmulate::new(Eval);
    let a = sys.constant(1234);
    let b = sys.constant(5678);
    let sum = sys.wrapping_add(&a, &b);
    let target = sys.constant(1234 + 5678);
    assert_eq!(sum, target);
}
