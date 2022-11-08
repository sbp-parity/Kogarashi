mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $inv:ident) => {
        ring_operation!($field, $p, $g, $e, $inv);

        impl Field for $field {}

        impl Div for $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: $field) -> $field {
                let inv = $field(invert(rhs.0, $p.0, $inv).unwrap());
                self * inv
            }
        }

        impl<'a, 'b> Div<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: &'b $field) -> $field {
                let inv = $field(invert(rhs.0, $p.0, $inv).unwrap());
                self * &inv
            }
        }

        impl DivAssign for $field {
            fn div_assign(&mut self, rhs: $field) {
                let inv = $field(invert(rhs.0, $p.0, $inv).unwrap());
                *self *= inv
            }
        }
    };
}

#[macro_export]
macro_rules! prime_field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $inv:ident) => {
        field_operation!($field, $p, $g, $e, $inv);

        built_in_operation!($field);

        impl PrimeField for $field {
            const INV: u64 = $inv;

            fn double(self) -> Self {
                Self(double(self.0, $p.0))
            }

            fn square(self) -> Self {
                Self(square(self.0, $p.0))
            }

            fn double_assign(&mut self) {
                self.0 = double(self.0, $p.0)
            }

            fn square_assign(&mut self) {
                self.0 = square(self.0, $p.0)
            }
        }
    };
}

#[macro_export]
macro_rules! fft_field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $i:ident, $r:ident) => {
        use zero_crypto::arithmetic::limbs::bits_256::*;

        prime_field_operation!($field, $p, $g, $e, $i);

        impl FftField for $field {
            const ROOT_OF_UNITY: Self = $r;
        }

        impl ParallelCmp for $field {}
    };
}

pub use field_operation;

pub use prime_field_operation;

pub use fft_field_operation;
