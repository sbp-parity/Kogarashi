use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use crate::params::{
    BLS_X, BLS_X_IS_NEGATIVE, FROBENIUS_COEFF_FQ12_C1, FROBENIUS_COEFF_FQ2_C1,
    FROBENIUS_COEFF_FQ6_C1,
};
use zero_crypto::dress::extension_field::*;
use zero_crypto::dress::pairing::{bls12_range_field_pairing, peculiar_extension_field_operation};

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 3;
extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENTION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
bls12_range_field_pairing!(Fq12, Fq2, G1Affine, PairingCoeff, BLS_X, BLS_X_IS_NEGATIVE);

// non common extension operation
peculiar_extension_field_operation!(
    Fq2,
    Fq6,
    Fq12,
    FROBENIUS_COEFF_FQ2_C1,
    FROBENIUS_COEFF_FQ6_C1,
    FROBENIUS_COEFF_FQ12_C1,
    BLS_X_IS_NEGATIVE
);

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn fq2_mul_nonresidue_test() {
        let b = Fq2([Fq::one(); TWO_DEGREE_EXTENTION_LIMBS_LENGTH]);
        for _ in 0..100 {
            let a = Fq2::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonresidue(), expected)
        }
    }

    #[test]
    fn fq6_mul_nonresidue_test() {
        let b = Fq6([Fq2::zero(), Fq2::one(), Fq2::zero()]);
        for _ in 0..100 {
            let a = Fq6::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonresidue(), expected)
        }
    }
}
