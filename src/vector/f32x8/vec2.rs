use crate::vector::{Comp, Ops, Vector, Vector2D};
use core::ops::*;
use std::simd::{f32x8, SimdFloat, StdFloat};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vec2x8 {
    pub x: f32x8,
    pub y: f32x8,
}

impl Vec2x8 {
    pub const ZERO: Self = Self::new_splat(0.0);
    pub const ONE: Self = Self::new_splat(1.0);

    #[inline(always)]
    pub const fn new(x: f32x8, y: f32x8) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(v: f32x8) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    pub const fn new_splat(v: f32) -> Self {
        Self {
            x: f32x8::from_array([v; 8]),
            y: f32x8::from_array([v; 8]),
        }
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f32x8 {
        (self.x * rhs.x) + (self.y * rhs.y)
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
    pub fn length_recip(self) -> f32x8 {
        self.length().recip()
    }

    #[must_use]
    #[inline]
    pub fn normalise(&self) -> Self {
        self.mul(self.length_recip())
    }

    #[inline]
    pub fn mul_add(&self, m: Self, a: Self) -> Self {
        Self::new(self.x.mul_add(m.x, a.x), self.y.mul_add(m.y, a.y))
    }

    #[inline]
    pub fn powf(&self, n: f32x8) -> Self {
        Self::new(super::powf(self.x, n), super::powf(self.y, n))
    }

    #[inline]
    pub fn min(&self, rhs: Self) -> Self {
        Self {
            x: self.x.simd_min(rhs.x),
            y: self.y.simd_min(rhs.y),
        }
    }

    #[inline]
    pub fn max(&self, rhs: Self) -> Self {
        Self {
            x: self.x.simd_max(rhs.x),
            y: self.y.simd_max(rhs.y),
        }
    }

    #[inline]
    pub fn min_element(&self) -> f32x8 {
        self.x.simd_min(self.y)
    }

    #[inline]
    pub fn max_element(&self) -> f32x8 {
        self.x.simd_max(self.y)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Ops for Vec2x8 {}
impl Ops<f32x8, Vec2x8> for Vec2x8 {}

impl Vector<2, f32x8> for Vec2x8 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;

    #[inline]
    fn dot(&self, other: Self) -> f32x8 {
        Vec2x8::dot(self, other)
    }

    #[inline]
    fn length(&self) -> f32x8 {
        Vec2x8::length(self)
    }

    #[inline]
    fn normalise(&self) -> Self {
        Vec2x8::normalise(self)
    }

    #[inline]
    fn mul_add(&self, m: Self, a: Self) -> Self {
        Vec2x8::mul_add(self, m, a)
    }

    #[inline]
    fn abs(&self) -> Self {
        Vec2x8::abs(self)
    }

    #[inline]
    fn max(&self, other: Self) -> Self {
        Vec2x8::max(self, other)
    }

    #[inline]
    fn max_element(&self) -> f32x8 {
        Vec2x8::max_element(self)
    }

    #[inline]
    fn min(&self, other: Self) -> Self {
        Vec2x8::min(self, other)
    }

    #[inline]
    fn min_element(&self) -> f32x8 {
        Vec2x8::min_element(self)
    }

    #[inline]
    fn powf(&self, exp: f32x8) -> Self {
        Vec2x8::powf(self, exp)
    }
}

impl Vector2D<f32x8> for Vec2x8 {
    fn x(&self) -> f32x8 {
        self.x
    }

    fn y(&self) -> f32x8 {
        self.y
    }
}

impl Comp<2> for f32x8 {
    type Vec = Vec2x8;

    #[inline]
    fn new_vec([x, y]: [Self; 2]) -> Self::Vec {
        Vec2x8::new(x, y)
    }
}

impl Div<Vec2x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl DivAssign<Vec2x8> for Vec2x8 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl Div<f32x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl DivAssign<f32x8> for Vec2x8 {
    #[inline]
    fn div_assign(&mut self, rhs: f32x8) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl Div<Vec2x8> for f32x8 {
    type Output = Vec2x8;
    #[inline]
    fn div(self, rhs: Vec2x8) -> Vec2x8 {
        Vec2x8 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Div<f32> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        self.div(f32x8::splat(rhs))
    }
}

impl DivAssign<f32> for Vec2x8 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.div_assign(f32x8::splat(rhs))
    }
}

impl Mul<Vec2x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl MulAssign<Vec2x8> for Vec2x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl Mul<f32x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl MulAssign<f32x8> for Vec2x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32x8) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl Mul<Vec2x8> for f32x8 {
    type Output = Vec2x8;
    #[inline]
    fn mul(self, rhs: Vec2x8) -> Vec2x8 {
        Vec2x8 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Mul<f32> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul(f32x8::splat(rhs))
    }
}

impl MulAssign<f32> for Vec2x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.mul_assign(f32x8::splat(rhs))
    }
}

impl Mul<Vec2x8> for f32 {
    type Output = Vec2x8;
    #[inline]
    fn mul(self, rhs: Vec2x8) -> Vec2x8 {
        f32x8::splat(self).mul(rhs)
    }
}

impl Add<Vec2x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl AddAssign<Vec2x8> for Vec2x8 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Add<f32x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl AddAssign<f32x8> for Vec2x8 {
    #[inline]
    fn add_assign(&mut self, rhs: f32x8) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl Add<Vec2x8> for f32x8 {
    type Output = Vec2x8;
    #[inline]
    fn add(self, rhs: Vec2x8) -> Vec2x8 {
        Vec2x8 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<Vec2x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl SubAssign<Vec2x8> for Vec2x8 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2x8) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl Sub<f32x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl SubAssign<f32x8> for Vec2x8 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32x8) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl Sub<Vec2x8> for f32x8 {
    type Output = Vec2x8;
    #[inline]
    fn sub(self, rhs: Vec2x8) -> Vec2x8 {
        Vec2x8 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Rem<Vec2x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl RemAssign<Vec2x8> for Vec2x8 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl Rem<f32x8> for Vec2x8 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32x8) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl RemAssign<f32x8> for Vec2x8 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32x8) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl Rem<Vec2x8> for f32x8 {
    type Output = Vec2x8;
    #[inline]
    fn rem(self, rhs: Vec2x8) -> Vec2x8 {
        Vec2x8 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl Neg for Vec2x8 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}
