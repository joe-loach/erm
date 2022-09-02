use crate::vector::{Comp, Ops, Vector, Vector3D};
use core::ops::*;
use std::simd::{f32x8, SimdFloat, StdFloat};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vec3x8 {
    pub x: f32x8,
    pub y: f32x8,
    pub z: f32x8,
}

impl Vec3x8 {
    pub const ZERO: Self = Self::new_splat(0.0);
    pub const ONE: Self = Self::new_splat(1.0);

    #[inline(always)]
    pub const fn new(x: f32x8, y: f32x8, z: f32x8) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(v: f32x8) -> Self {
        Self { x: v, y: v, z: v }
    }

    #[inline]
    pub const fn new_splat(v: f32) -> Self {
        Self {
            x: f32x8::from_array([v; 8]),
            y: f32x8::from_array([v; 8]),
            z: f32x8::from_array([v; 8]),
        }
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f32x8 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    #[inline]
    pub fn length(&self) -> f32x8 {
        self.length_sq().sqrt()
    }

    #[inline]
    pub fn length_sq(&self) -> f32x8 {
        self.dot(*self)
    }

    #[inline]
    pub fn length_recip(&self) -> f32x8 {
        self.length().recip()
    }

    #[must_use]
    #[inline]
    pub fn normalise(&self) -> Self {
        self.mul(self.length_recip())
    }

    #[inline]
    pub fn mul_add(&self, m: Self, a: Self) -> Self {
        Self::new(
            self.x.mul_add(m.x, a.x),
            self.y.mul_add(m.y, a.y),
            self.z.mul_add(m.z, a.z),
        )
    }

    #[inline]
    pub fn powf(&self, n: f32x8) -> Self {
        Self::new(
            super::powf(self.x, n),
            super::powf(self.y, n),
            super::powf(self.z, n),
        )
    }

    #[inline]
    pub fn min(&self, rhs: Self) -> Self {
        Self {
            x: self.x.simd_min(rhs.x),
            y: self.y.simd_min(rhs.y),
            z: self.z.simd_min(rhs.z),
        }
    }

    #[inline]
    pub fn max(&self, rhs: Self) -> Self {
        Self {
            x: self.x.simd_max(rhs.x),
            y: self.y.simd_max(rhs.y),
            z: self.z.simd_max(rhs.z),
        }
    }

    #[inline]
    pub fn min_element(&self) -> f32x8 {
        self.x.simd_min(self.y.simd_min(self.z))
    }

    #[inline]
    pub fn max_element(&self) -> f32x8 {
        self.x.simd_max(self.y.simd_max(self.z))
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl Ops for Vec3x8 {}
impl Ops<f32x8, Vec3x8> for Vec3x8 {}

impl Vector<3, f32x8> for Vec3x8 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;

    #[inline]
    fn dot(&self, other: Self) -> f32x8 {
        Vec3x8::dot(self, other)
    }

    #[inline]
    fn length(&self) -> f32x8 {
        Vec3x8::length(self)
    }

    #[inline]
    fn normalise(&self) -> Self {
        Vec3x8::normalise(self)
    }

    #[inline]
    fn mul_add(&self, m: Self, a: Self) -> Self {
        Vec3x8::mul_add(self, m, a)
    }

    #[inline]
    fn abs(&self) -> Self {
        Vec3x8::abs(self)
    }

    #[inline]
    fn max(&self, other: Self) -> Self {
        Vec3x8::max(self, other)
    }

    #[inline]
    fn max_element(&self) -> f32x8 {
        Vec3x8::max_element(self)
    }

    #[inline]
    fn min(&self, other: Self) -> Self {
        Vec3x8::min(self, other)
    }

    #[inline]
    fn min_element(&self) -> f32x8 {
        Vec3x8::min_element(self)
    }

    #[inline]
    fn powf(&self, exp: f32x8) -> Self {
        Vec3x8::powf(self, exp)
    }
}

impl Vector3D<f32x8> for Vec3x8 {
    fn x(&self) -> f32x8 {
        self.x
    }

    fn y(&self) -> f32x8 {
        self.y
    }

    fn z(&self) -> f32x8 {
        self.z
    }
}

impl Comp<3> for f32x8 {
    type Vec = Vec3x8;

    #[inline]
    fn new_vec([x, y, z]: [Self; 3]) -> Self::Vec {
        Vec3x8::new(x, y, z)
    }
}

impl Div<Vec3x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}

impl DivAssign<Vec3x8> for Vec3x8 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}

impl Div<f32x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

impl DivAssign<f32x8> for Vec3x8 {
    #[inline]
    fn div_assign(&mut self, rhs: f32x8) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}

impl Div<Vec3x8> for f32x8 {
    type Output = Vec3x8;
    #[inline]
    fn div(self, rhs: Vec3x8) -> Vec3x8 {
        Vec3x8 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}

impl Div<f32> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        self.div(f32x8::splat(rhs))
    }
}

impl DivAssign<f32> for Vec3x8 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.div_assign(f32x8::splat(rhs))
    }
}

impl Mul<Vec3x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}

impl MulAssign<Vec3x8> for Vec3x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}

impl Mul<f32x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl MulAssign<f32x8> for Vec3x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32x8) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}

impl Mul<Vec3x8> for f32x8 {
    type Output = Vec3x8;
    #[inline]
    fn mul(self, rhs: Vec3x8) -> Vec3x8 {
        Vec3x8 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}

impl Add<Vec3x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Mul<f32> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul(f32x8::splat(rhs))
    }
}

impl MulAssign<f32> for Vec3x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.mul_assign(f32x8::splat(rhs))
    }
}

impl Mul<Vec3x8> for f32 {
    type Output = Vec3x8;
    #[inline]
    fn mul(self, rhs: Vec3x8) -> Vec3x8 {
        f32x8::splat(self).mul(rhs)
    }
}

impl AddAssign<Vec3x8> for Vec3x8 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl Add<f32x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}

impl AddAssign<f32x8> for Vec3x8 {
    #[inline]
    fn add_assign(&mut self, rhs: f32x8) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}

impl Add<Vec3x8> for f32x8 {
    type Output = Vec3x8;
    #[inline]
    fn add(self, rhs: Vec3x8) -> Vec3x8 {
        Vec3x8 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}

impl Add<f32> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        self.add(f32x8::splat(rhs))
    }
}

impl AddAssign<f32> for Vec3x8 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.add_assign(f32x8::splat(rhs))
    }
}

impl Add<Vec3x8> for f32 {
    type Output = Vec3x8;
    #[inline]
    fn add(self, rhs: Vec3x8) -> Vec3x8 {
        f32x8::splat(self).add(rhs)
    }
}

impl Sub<Vec3x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl SubAssign<Vec3x8> for Vec3x8 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3x8) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl Sub<f32x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}

impl SubAssign<f32x8> for Vec3x8 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32x8) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl Sub<Vec3x8> for f32x8 {
    type Output = Vec3x8;
    #[inline]
    fn sub(self, rhs: Vec3x8) -> Vec3x8 {
        Vec3x8 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}

impl Rem<Vec3x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
            z: self.z.rem(rhs.z),
        }
    }
}

impl RemAssign<Vec3x8> for Vec3x8 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
    }
}

impl Rem<f32x8> for Vec3x8 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
            z: self.z.rem(rhs),
        }
    }
}

impl RemAssign<f32x8> for Vec3x8 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32x8) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
        self.z.rem_assign(rhs);
    }
}

impl Rem<Vec3x8> for f32x8 {
    type Output = Vec3x8;
    #[inline]
    fn rem(self, rhs: Vec3x8) -> Vec3x8 {
        Vec3x8 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
            z: self.rem(rhs.z),
        }
    }
}

impl Neg for Vec3x8 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}
