use crate::poly::Polynomial;
use rayon::{join, prelude::*};
use zero_crypto::common::*;

// fft structure
#[derive(Clone, Debug)]
pub struct Fft<F: FftField> {
    // polynomial degree 2^k
    k: usize,
    // generator of order 2^{k - 1} multiplicative group used as twiddle factors
    twiddle_factors: Vec<F>,
    // multiplicative group generator inverse
    inv_twiddle_factors: Vec<F>,
    // n inverse for inverse discrete fourier transform
    n_inv: F,
    // bit reverse index
    bit_reverse: Vec<usize>,
}

impl<F: FftField> Fft<F> {
    pub fn new(k: usize) -> Self {
        let n = 1 << k;
        let half_n = n / 2;
        let offset = 64 - k;

        // precompute twiddle factors
        let g = (0..F::S - k).fold(F::ROOT_OF_UNITY, |acc, _| acc.square());
        let twiddle_factors = (0..half_n as usize)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g;
                Some(tw)
            })
            .collect::<Vec<_>>();

        // precompute inverse twiddle factors
        let g_inv = g.invert().unwrap();
        let inv_twiddle_factors = (0..half_n as usize)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g_inv;
                Some(tw)
            })
            .collect::<Vec<_>>();

        Fft {
            k,
            twiddle_factors,
            inv_twiddle_factors,
            n_inv: F::from(n).invert().unwrap(),
            bit_reverse: (0..n as usize)
                .map(|i| i.reverse_bits() >> offset)
                .collect::<Vec<_>>(),
        }
    }

    // perform classic discrete fourier transform
    pub fn dft(&self, coeffs: &mut Polynomial<F>) {
        let n = 1 << self.k;
        assert_eq!(coeffs.len(), n);

        self.reverse_index(coeffs);
        classic_fft_arithmetic(coeffs, n, 1, &self.twiddle_factors)
    }

    // perform classic inverse discrete fourier transform
    pub fn idft(&self, coeffs: &mut Polynomial<F>) {
        let n = 1 << self.k;
        assert_eq!(coeffs.len(), n);

        self.reverse_index(coeffs);
        classic_fft_arithmetic(coeffs, n, 1, &self.inv_twiddle_factors);
        coeffs.par_iter_mut().for_each(|coeff| *coeff *= self.n_inv)
    }

    // polynomial coefficients bit reverse permutation
    fn reverse_index(&self, coeffs: &mut Polynomial<F>) {
        for (i, ri) in self.bit_reverse.iter().enumerate() {
            if i < *ri {
                coeffs.swap(*ri, i);
            }
        }
    }
}

// classic fft using divide and conquer algorithm
fn classic_fft_arithmetic<F: FftField>(
    coeffs: &mut Polynomial<F>,
    n: usize,
    twiddle_chunk: usize,
    twiddles: &Polynomial<F>,
) {
    if n == 2 {
        let t = coeffs[1];
        coeffs[1] = coeffs[0];
        coeffs[0] += t;
        coeffs[1] -= t;
    } else {
        let (left, right) = coeffs.split_at_mut(n / 2);
        join(
            || classic_fft_arithmetic(left, n / 2, twiddle_chunk * 2, twiddles),
            || classic_fft_arithmetic(right, n / 2, twiddle_chunk * 2, twiddles),
        );
        butterfly_arithmetic(left, right, twiddle_chunk, twiddles)
    }
}

pub(crate) fn butterfly_arithmetic<F: FftField>(
    left: &mut Polynomial<F>,
    right: &mut Polynomial<F>,
    twiddle_chunk: usize,
    twiddles: &Polynomial<F>,
) {
    // case when twiddle factor is one
    let t = right[0];
    right[0] = left[0];
    left[0] += t;
    right[0] -= t;

    left.iter_mut()
        .zip(right.iter_mut())
        .enumerate()
        .skip(1)
        .for_each(|(i, (a, b))| {
            let mut t = *b;
            t *= twiddles[i * twiddle_chunk];
            *b = *a;
            *a += t;
            *b -= t;
        });
}

#[cfg(test)]
mod tests {
    use super::Fft;
    use proptest::prelude::*;
    use proptest::std_facade::vec;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_bls12_381::Fr;
    use zero_crypto::behave::PrimeField;
    use zero_crypto::common::Vec;

    prop_compose! {
        fn arb_poly(k: u32)(bytes in vec![[any::<u8>(); 16]; 1 << k as usize]) -> Vec<Fr> {
            (0..(1 << k)).map(|i| Fr::random(XorShiftRng::from_seed(bytes[i]))).collect::<Vec<Fr>>()
        }
    }

    fn naive_multiply<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> Vec<F> {
        assert_eq!(a.len(), b.len());
        let mut c = vec![F::default(); a.len() + b.len()];
        a.iter().enumerate().for_each(|(i_a, coeff_a)| {
            b.iter().enumerate().for_each(|(i_b, coeff_b)| {
                c[i_a + i_b] += *coeff_a * *coeff_b;
            })
        });
        c
    }

    fn point_mutiply<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> Vec<F> {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(coeff_a, coeff_b)| *coeff_a * *coeff_b)
            .collect::<Vec<F>>()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1))]
        #[test]
        fn fft_transformation_test(mut poly_a in arb_poly(10)) {
            let poly_b = poly_a.clone();
            let classic_fft = Fft::new(10);

            classic_fft.dft(&mut poly_a);
            classic_fft.idft(&mut poly_a);

            assert_eq!(poly_a, poly_b)
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn fft_multiplication_test(mut poly_a in arb_poly(4), mut poly_b in arb_poly(4)) {
            let fft = Fft::new(5);
            let poly_c = poly_a.clone();
            let poly_d = poly_b.clone();
            poly_a.resize(1<<5, Fr::zero());
            poly_b.resize(1<<5, Fr::zero());

            let poly_e = naive_multiply(poly_c, poly_d);

            fft.dft(&mut poly_a);
            fft.dft(&mut poly_b);
            let mut poly_f = point_mutiply(poly_a, poly_b);
            fft.idft(&mut poly_f);

            assert_eq!(poly_e.len(), poly_f.len());
            assert_eq!(poly_e, poly_f)
        }
    }
}
