// This trait resresents prime field

use super::{algebra::Field, basic::Basic, comp::ParityCmp};

use rand_core::RngCore;

/// This is prime field trait
pub trait PrimeField: Field + Basic + ParityCmp + PartialOrd + Ord {
    const MODULUS: Self;

    // mongomery reduction inverse
    const INV: u64;

    fn from_u64(val: u64) -> Self;

    fn from_u512(val: [u64; 8]) -> Self;

    fn to_bits(self) -> [u8; 256];

    fn is_zero(self) -> bool;

    fn random(rand: impl RngCore) -> Self;

    fn double(self) -> Self;

    fn square(self) -> Self;

    fn double_assign(&mut self);

    fn square_assign(&mut self);
}
