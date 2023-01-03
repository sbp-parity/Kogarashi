// This trait resresents prime field

use core::fmt::Debug;

use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
};
use crate::arithmetic::utils::Bits;

/// This is prime field trait
pub trait PrimeField: Field + Basic + ParityCmp {
    // prime order of this field
    const MODULUS: Self;

    // mongomery reduction inverse
    const INV: u64;

    fn is_zero(self) -> bool;

    fn to_bits(self) -> Bits;

    fn double(self) -> Self;

    fn square(self) -> Self;

    fn double_assign(&mut self);

    fn square_assign(&mut self);
}

pub trait FieldRepr: Debug {
    const LIMBS_LENGTH: usize;

    // map from montgomery to normal form
    fn to_repr(self) -> Self;
}
