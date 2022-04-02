use std::ops::*;

/// A system in which values of type `T` can be represented.
pub trait SystemRepr<T> {
    /// An abstract representation of a value of a certain type in the system.
    type Abstract: Clone;

    /// Constructs an [`Abstract`] wrapper over the given constant value.
    fn constant(&mut self, value: T) -> Self::Abstract;
}

/// An abstract representation of a value of type `T` within a system of type `S`.
pub type Abstract<S, T> = <S as SystemRepr<T>>::Abstract;

/// A system in which abstract values of type `T` can be added together. If an implementation of
/// [`Add`] exists for `T`, this must be consistent with it.
pub trait SystemAdd<T>: SystemRepr<T> {
    fn add(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T>;
}

/// A system in which abstract values of type `T` can be added together, with wrapping.
pub trait SystemWrappingAdd<T>: SystemRepr<T> {
    fn wrapping_add(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T>;
}

/// A system in which abstract values of type `T` can be bitwise-ANDed together. If an
/// implementation of [`BitAnd`] exists for `T`, this must be consistent with it.
pub trait SystemBitAnd<T>: SystemRepr<T> {
    fn and(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T>;
}

/// A system in which abstract values of type `T` can be bitwise-ORed together. If an
/// implementation of [`BitOr`] exists for `T`, this must be consistent with it.
pub trait SystemBitOr<T>: SystemRepr<T> {
    fn or(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T>;
}

/// A system in which abstract values of type `T` can be bitwise-XORed together. If an
/// implementation of [`BitXor`] exists for `T`, this must be consistent with it.
pub trait SystemBitXor<T>: SystemRepr<T> {
    fn xor(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T>;
}

/// A system in which abstract values of type `A` can be bit-shifted by constant values
/// of type `B`. If an implementation of [`Shl`] or [`Shr`] exists for `A`, this must be consistent
/// with it.
pub trait SystemBitShift<A, B>: SystemRepr<A> {
    fn shl(&mut self, a: &Abstract<Self, A>, b: B) -> Abstract<Self, A>;
    fn shr(&mut self, a: &Abstract<Self, A>, b: B) -> Abstract<Self, A>;
}

/// A system in which abstract values of type `A` can be bitwise-rotated by constant values
/// of type `B`.
pub trait SystemBitRotate<A, B>: SystemRepr<A> {
    fn rotl(&mut self, a: &Abstract<Self, A>, b: B) -> Abstract<Self, A>;
    fn rotr(&mut self, a: &Abstract<Self, A>, b: B) -> Abstract<Self, A>;
}

/// A system in which abstract values of type `T` can be bitwise/logically negated. If an
/// implementation of [`Not`] exists for `T`, this must be consistent with it.
pub trait SystemNot<T>: SystemRepr<T> {
    fn not(&mut self, value: &Abstract<Self, T>) -> Abstract<Self, T>;
}

/// A system in which abstract boolean values can be "asserted".
pub trait SystemAssert: SystemRepr<bool> {
    /// Asserts that the given value is true. For constraint systems, this imposes a constraint,
    /// whereas for evaluation systems, this may panic if the assertion is false.
    fn assert(&mut self, value: &Abstract<Self, bool>);
}

/// A system in which abstract boolean values can be "asserted".
pub trait SystemAssertEq<T>: SystemRepr<T> {
    /// Asserts that the given values are equal. For constraint systems, this imposes a constraint,
    /// whereas for evaluation systems, this may panic if the assertion is false.
    fn assert_eq(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>);
}

impl<S: SystemRepr<T> + ?Sized, T, const N: usize> SystemRepr<[T; N]> for S {
    type Abstract = [Abstract<S, T>; N];
    fn constant(&mut self, value: [T; N]) -> Self::Abstract {
        value.map(|v| self.constant(v))
    }
}

/// A "system" that directly evaluates values.
pub struct Eval;

impl SystemRepr<bool> for Eval {
    type Abstract = bool;
    fn constant(&mut self, value: bool) -> bool {
        value
    }
}

impl SystemRepr<u32> for Eval {
    type Abstract = u32;
    fn constant(&mut self, value: u32) -> u32 {
        value
    }
}

impl<T> SystemAdd<T> for Eval
where
    for<'a, 'b> &'a T: Add<&'b T, Output = T>,
    Eval: SystemRepr<T, Abstract = T>,
{
    fn add(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T> {
        a + b
    }
}

impl SystemWrappingAdd<u32> for Eval {
    fn wrapping_add(
        &mut self,
        a: &Abstract<Self, u32>,
        b: &Abstract<Self, u32>,
    ) -> Abstract<Self, u32> {
        a.wrapping_add(*b)
    }
}

impl<T> SystemBitAnd<T> for Eval
where
    for<'a, 'b> &'a T: BitAnd<&'b T, Output = T>,
    Eval: SystemRepr<T, Abstract = T>,
{
    fn and(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T> {
        a & b
    }
}

impl<T> SystemBitOr<T> for Eval
where
    for<'a, 'b> &'a T: BitOr<&'b T, Output = T>,
    Eval: SystemRepr<T, Abstract = T>,
{
    fn or(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T> {
        a | b
    }
}

impl<T> SystemBitXor<T> for Eval
where
    for<'a, 'b> &'a T: BitXor<&'b T, Output = T>,
    Eval: SystemRepr<T, Abstract = T>,
{
    fn xor(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) -> Abstract<Self, T> {
        a ^ b
    }
}

impl<A, B> SystemBitShift<A, B> for Eval
where
    for<'a> &'a A: Shl<B, Output = A>,
    for<'a> &'a A: Shr<B, Output = A>,
    Eval: SystemRepr<A, Abstract = A>,
{
    fn shl(&mut self, a: &Abstract<Self, A>, b: B) -> Abstract<Self, A> {
        a << b
    }

    fn shr(&mut self, a: &Abstract<Self, A>, b: B) -> Abstract<Self, A> {
        a >> b
    }
}

impl SystemBitRotate<u32, u8> for Eval {
    fn rotl(&mut self, a: &Abstract<Self, u32>, b: u8) -> Abstract<Self, u32> {
        a.rotate_left(b as u32)
    }

    fn rotr(&mut self, a: &Abstract<Self, u32>, b: u8) -> Abstract<Self, u32> {
        a.rotate_right(b as u32)
    }
}

impl<T> SystemNot<T> for Eval
where
    for<'a> &'a T: Not<Output = T>,
    Eval: SystemRepr<T, Abstract = T>,
{
    fn not(&mut self, value: &Abstract<Self, T>) -> Abstract<Self, T> {
        !value
    }
}

impl SystemAssert for Eval {
    fn assert(&mut self, value: &Abstract<Self, bool>) {
        assert!(value)
    }
}

impl<T> SystemAssertEq<T> for Eval
where
    for<'a> &'a T: Eq,
    Eval: SystemRepr<T, Abstract = T>
{
    fn assert_eq(&mut self, a: &Abstract<Self, T>, b: &Abstract<Self, T>) {
        assert!(a == b)
    }
}
