mod vec2;
mod vec3;

pub use vec2::Vec2;
pub use vec3::Vec3;

use super::{CompOps, FromFloat, Ops};

impl Ops for f32 {}

impl CompOps for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    #[inline]
    fn min(&self, other: Self) -> Self {
        f32::min(*self, other)
    }

    #[inline]
    fn max(&self, other: Self) -> Self {
        f32::max(*self, other)
    }

    #[inline]
    fn clamp(&self, min: Self, max: Self) -> Self {
        f32::clamp(*self, min, max)
    }

    #[inline]
    fn powf(&self, exp: Self) -> Self {
        f32::powf(*self, exp)
    }
}

impl FromFloat for f32 {
    fn from(v: f32) -> Self {
        v
    }
}
