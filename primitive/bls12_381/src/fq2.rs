use crate::fq::Fq;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::dress::extention_field::*;

// quadratic finite field struct
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fq2(pub(crate) [Fq; 2]);

const ZERO: Fq2 = Fq2([Fq([0, 0, 0, 0, 0, 0]), Fq([0, 0, 0, 0, 0, 0])]);

extention_field_operation!(Fq2, Fq, ZERO);
