mod vec2;
mod vec3;

pub use vec2::Vec2x8;
pub use vec3::Vec3x8;

use super::{CompOps, FromFloat, Ops};
use std::simd::{f32x8, SimdFloat};

impl Ops for f32x8 {}

impl CompOps for f32x8 {
    const ZERO: Self = f32x8::from_array([0.0; 8]);
    const ONE: Self = f32x8::from_array([1.0; 8]);

    #[inline]
    fn min(&self, other: Self) -> Self {
        f32x8::simd_min(*self, other)
    }

    #[inline]
    fn max(&self, other: Self) -> Self {
        f32x8::simd_max(*self, other)
    }

    #[inline]
    fn clamp(&self, min: Self, max: Self) -> Self {
        f32x8::simd_clamp(*self, min, max)
    }

    #[inline]
    fn powf(&self, exp: Self) -> Self {
        powf(*self, exp)
    }
}

impl FromFloat for std::simd::f32x8 {
    fn from(v: f32) -> Self {
        std::simd::f32x8::splat(v)
    }
}

/// Computes `x` raised to the power of `y`
fn powf(x: f32x8, y: f32x8) -> f32x8 {
    // it is really unfortunate that we have to use platform instrinsics here.
    // however, as of writing, there is no `f32x8::powf` function.
    extern "platform-intrinsic" {
        fn simd_fpow<T>(x: T, y: T) -> T;
    }
    // SAFETY:
    // Honestly, I can't find documentation for platform instrinsics apart from a symbol table.
    // I trust the compiler to emit the correct instructions :)
    unsafe { simd_fpow(x, y) }
}
