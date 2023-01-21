#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x == other.x && self.y == other.y
                }
            }
        }

        impl Eq for $affine {}

        impl Default for $affine {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl $affine {
            pub const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                is_infinity: false,
            };

            pub const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
                is_infinity: true,
            };

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

            fn invert(self) -> Option<Self> {
                match self.is_infinity {
                    true => None,
                    false => Some(Self {
                        x: self.x,
                        y: -self.y,
                        is_infinity: false,
                    }),
                }
            }

            pub fn random(rand: impl RngCore) -> $projective {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl Add for $affine {
            type Output = $projective;

            fn add(self, rhs: $affine) -> Self::Output {
                $projective::from(add_point(self.to_projective(), rhs.to_projective()))
            }
        }

        impl Neg for $affine {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    is_infinity: self.is_infinity,
                }
            }
        }

        impl Sub for $affine {
            type Output = $projective;

            fn sub(self, rhs: $affine) -> Self::Output {
                $projective::from(add_point(self.to_projective(), rhs.neg().to_projective()))
            }
        }

        impl Mul<<Self as Affine>::Scalar> for $affine {
            type Output = $projective;

            fn mul(self, rhs: <Self as Affine>::Scalar) -> Self::Output {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.to_projective();
                for &naf in rhs.to_nafs().iter().rev() {
                    if naf == Naf::Plus {
                        res += acc;
                    } else if naf == Naf::Minus {
                        res -= acc;
                    }
                    acc = acc.double();
                }
                res
            }
        }

        impl<'b> Mul<&'b <Self as Affine>::Scalar> for $affine {
            type Output = $projective;

            fn mul(self, rhs: &'b <Self as Affine>::Scalar) -> Self::Output {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.to_projective();
                for &naf in rhs.to_nafs().iter().rev() {
                    if naf == Naf::Plus {
                        res += acc;
                    } else if naf == Naf::Minus {
                        res -= acc;
                    }
                    acc = acc.double();
                }
                res
            }
        }
    };
}

#[macro_export]
macro_rules! projective_group_operation {
    ($projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        curve_arithmetic_extension!($projective, $scalar);

        impl Group for $projective {
            type Scalar = $scalar;

            const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                z: $range::one(),
            };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
                z: $range::zero(),
            };

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

            fn invert(self) -> Option<Self> {
                match self.z.is_zero() {
                    true => None,
                    false => Some(Self {
                        x: self.x,
                        y: -self.y,
                        z: self.z,
                    }),
                }
            }

            fn random(rand: impl RngCore) -> Self {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl PartialEq for $projective {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z
                }
            }
        }

        impl Add for $projective {
            type Output = Self;

            fn add(self, rhs: $projective) -> Self {
                add_point(self, rhs)
            }
        }

        impl Neg for $projective {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    z: self.z,
                }
            }
        }

        impl Sub for $projective {
            type Output = Self;

            fn sub(self, rhs: $projective) -> Self {
                add_point(self, -rhs)
            }
        }

        impl Mul<<Self as Group>::Scalar> for $projective {
            type Output = Self;

            fn mul(self, rhs: <Self as Group>::Scalar) -> Self {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.clone();
                for &naf in rhs.to_nafs().iter().rev() {
                    if naf == Naf::Plus {
                        res += acc;
                    } else if naf == Naf::Minus {
                        res -= acc;
                    }
                    acc = acc.double();
                }
                res
            }
        }

        impl<'b> Mul<&'b <Self as Group>::Scalar> for $projective {
            type Output = $projective;

            fn mul(self, rhs: &'b <Self as Group>::Scalar) -> $projective {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.clone();
                for &naf in rhs.to_nafs().iter().rev() {
                    if naf == Naf::Plus {
                        res += acc;
                    } else if naf == Naf::Minus {
                        res -= acc;
                    }
                    acc = acc.double();
                }
                res
            }
        }
    };
}

#[macro_export]
macro_rules! curve_arithmetic_extension {
    ($curve:ident, $scalar:ident) => {
        impl Eq for $curve {}

        impl Default for $curve {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl AddAssign for $curve {
            fn add_assign(&mut self, rhs: $curve) {
                *self = *self + rhs;
            }
        }

        impl<'b> AddAssign<&'b $curve> for $curve {
            fn add_assign(&mut self, rhs: &'b $curve) {
                *self = &*self + rhs;
            }
        }

        impl<'a, 'b> Add<&'b $curve> for &'a $curve {
            type Output = $curve;

            fn add(self, rhs: &'b $curve) -> $curve {
                self + rhs
            }
        }

        impl<'b> Add<&'b $curve> for $curve {
            type Output = $curve;

            fn add(self, rhs: &'b $curve) -> Self {
                &self + rhs
            }
        }

        impl<'a> Add<$curve> for &'a $curve {
            type Output = $curve;

            fn add(self, rhs: $curve) -> $curve {
                self + rhs
            }
        }

        impl SubAssign for $curve {
            fn sub_assign(&mut self, rhs: $curve) {
                *self = *self - rhs;
            }
        }

        impl<'b> SubAssign<&'b $curve> for $curve {
            fn sub_assign(&mut self, rhs: &'b $curve) {
                *self = &*self - rhs;
            }
        }

        impl<'a, 'b> Sub<&'b $curve> for &'a $curve {
            type Output = $curve;

            fn sub(self, rhs: &'b $curve) -> $curve {
                self - rhs
            }
        }

        impl<'b> Sub<&'b $curve> for $curve {
            type Output = $curve;

            fn sub(self, rhs: &'b $curve) -> Self {
                self - rhs
            }
        }

        impl<'a> Sub<$curve> for &'a $curve {
            type Output = $curve;

            fn sub(self, rhs: $curve) -> $curve {
                self - rhs
            }
        }

        impl MulAssign<<Self as Group>::Scalar> for $curve {
            fn mul_assign(&mut self, rhs: <Self as Group>::Scalar) {
                *self = *self * rhs;
            }
        }
    };
}

pub use {affine_group_operation, curve_arithmetic_extension, projective_group_operation};
