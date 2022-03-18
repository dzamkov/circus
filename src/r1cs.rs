use crate::*;
use ff::{Field, PrimeField};
use std::cmp::max;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

/// An indexed variable within a constraint system.
pub struct Variable(usize);

/// A linear combination of indexed variables.
#[derive(Clone)]
pub struct LinearFormula<F> {
    constant_term: F,
    coeffs: BTreeMap<usize, F>,
}

impl<F> LinearFormula<F> {
    /// Constructs a [`LinearFormula`] with the given constant value.
    pub fn constant(value: F) -> Self {
        LinearFormula {
            constant_term: value,
            coeffs: BTreeMap::new(),
        }
    }

    /// One more than the last variable index referenced in this formula.
    pub fn dim(&self) -> usize {
        self.coeffs.keys().rev().next().map(|k| k + 1).unwrap_or(0)
    }
}

impl<F: Add<F, Output = F>> Add<&LinearFormula<F>> for &LinearFormula<F> {
    type Output = LinearFormula<F>;
    fn add(self, rhs: &LinearFormula<F>) -> Self::Output {
        todo!()
    }
}

impl<F: Add<F, Output = F>> Sub<&LinearFormula<F>> for &LinearFormula<F> {
    type Output = LinearFormula<F>;
    fn sub(self, rhs: &LinearFormula<F>) -> Self::Output {
        todo!()
    }
}

/// A constraint which specifies that the product of two [`LinearFormula`] is a particular
/// [`LinearFormula`].
pub struct ProductConstraint<F> {
    pub operand_a: LinearFormula<F>,
    pub operand_b: LinearFormula<F>,
    pub result: LinearFormula<F>,
}

impl<F> ProductConstraint<F> {
    /// One more than the last variable index referenced in this constraint.
    pub fn dim(&self) -> usize {
        max(
            max(self.operand_a.dim(), self.operand_b.dim()),
            self.result.dim(),
        )
    }
}

/// A constraint system consisting of [`ProductConstraint`]s.
pub struct ArithmeticSystem<F> {
    num_vars: usize,
    constraints: Vec<ProductConstraint<F>>,
}

impl<F> ArithmeticSystem<F> {
    /// Constructs a new [`ArithmeticSystem`].
    pub fn new() -> Self {
        ArithmeticSystem {
            num_vars: 0,
            constraints: Vec::new(),
        }
    }

    /// Declares a new variable in this system.
    pub fn declare(&mut self) -> Variable {
        let index = self.num_vars;
        self.num_vars += 1;
        Variable(index)
    }

    /// Introduces a constraint into this system.
    pub fn satisfy(&mut self, constraint: ProductConstraint<F>) {
        assert!(constraint.dim() <= self.num_vars);
        self.constraints.push(constraint)
    }
}

impl<F: Field> SystemRepr<bool> for ArithmeticSystem<F> {
    type Abstract = LinearFormula<F>;
    fn constant(&mut self, value: bool) -> Self::Abstract {
        LinearFormula::constant(if value { F::one() } else { F::zero() })
    }
}

impl<F: Field> SystemBitAnd<bool> for ArithmeticSystem<F> {
    fn and(&mut self, a: &Abstract<Self, bool>, b: &Abstract<Self, bool>) -> Abstract<Self, bool> {
        todo!()
    }
}

impl<F: Field> SystemBitOr<bool> for ArithmeticSystem<F> {
    fn or(&mut self, a: &Abstract<Self, bool>, b: &Abstract<Self, bool>) -> Abstract<Self, bool> {
        let a = self.not(a);
        let b = self.not(b);
        let r = self.and(&a, &b);
        self.not(&r)
    }
}

impl<F: Field> SystemBitXor<bool> for ArithmeticSystem<F> {
    fn xor(&mut self, a: &Abstract<Self, bool>, b: &Abstract<Self, bool>) -> Abstract<Self, bool> {
        if (F::one() + F::one()).is_zero_vartime() {
            a + b
        } else {
            todo!()
        }
    }
}

impl<F: Field> SystemNot<bool> for ArithmeticSystem<F> {
    fn not(&mut self, value: &Abstract<Self, bool>) -> Abstract<Self, bool> {
        &LinearFormula::constant(F::one()) - value
    }
}

// TODO: Require `F::CAPACITY >= 8`
impl<F: PrimeField> SystemRepr<u8> for ArithmeticSystem<F> {
    type Abstract = LinearFormula<F>;
    fn constant(&mut self, value: u8) -> Self::Abstract {
        LinearFormula::constant(F::from(u64::from(value)))
    }
}
