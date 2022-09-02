mod f32;
mod f32x8;

use core::ops::*;

pub mod prelude {
    pub use super::f32::*;
    pub use super::f32x8::*;
    pub use super::{broadcast, vec2, vec3};
    pub use super::{Comp, Maskable, Vector, Vector2D, Vector3D};
}

pub trait Ops<I = Self, O = Self>:
    Sized
    + Add<I, Output = O>
    + AddAssign<I>
    + Sub<I, Output = O>
    + SubAssign<I>
    + Mul<I, Output = O>
    + MulAssign<I>
    + Div<I, Output = O>
    + DivAssign<I>
    + Rem<I, Output = O>
    + RemAssign<I>
    + Neg<Output = O>
{
}

pub trait CompOps: Ops {
    const ZERO: Self;
    const ONE: Self;

    fn min(&self, other: Self) -> Self;
    fn max(&self, other: Self) -> Self;
    fn clamp(&self, min: Self, max: Self) -> Self;
    fn powf(&self, exp: Self) -> Self;
}

/// An N dimensional Vector containing components of type T.
pub trait Vector<const N: usize, T>
where
    Self: Sized + Copy + Send + Sync + Ops + Ops<T, Self>,
    T: Comp<N>,
{
    const ZERO: Self;
    const ONE: Self;

    fn dot(&self, other: Self) -> T;
    fn length(&self) -> T;
    fn normalise(&self) -> Self;
    fn mul_add(&self, m: Self, a: Self) -> Self;
    fn abs(&self) -> Self;
    fn max(&self, other: Self) -> Self;
    fn max_element(&self) -> T;
    fn min(&self, other: Self) -> Self;
    fn min_element(&self) -> T;
    fn powf(&self, exp: T) -> Self;
}

pub trait Vector2D<T>: Vector<2, T>
where
    T: Comp<2>,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
}

pub trait Vector3D<T>: Vector<3, T>
where
    T: Comp<3>,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

/// A component of an N dimensional vector.
pub trait Comp<const N: usize>: Sized + Copy + Ops + CompOps + Send + Sync + FromFloat {
    type Vec: Vector<N, Self>;

    /// Create a new vector from an array of it's component.
    // Rust doesn't support varadic function arguments, so has to take an array.
    #[doc(hidden)]
    fn new_vec(arr: [Self; N]) -> Self::Vec;
}

pub trait FromFloat {
    fn from(v: f32) -> Self;
}

/// A mask type
pub trait Maskable<T> {
    fn select(&self, true_values: T, false_values: T) -> T;
}

impl<T> Maskable<T> for bool {
    #[inline]
    fn select(&self, true_values: T, false_values: T) -> T {
        if *self {
            true_values
        } else {
            false_values
        }
    }
}

impl<T, M, const LANES: usize> Maskable<std::simd::Simd<T, LANES>> for std::simd::Mask<M, LANES>
where
    std::simd::LaneCount<LANES>: std::simd::SupportedLaneCount,
    M: std::simd::MaskElement,
    T: std::simd::SimdElement<Mask = M>,
{
    #[inline]
    fn select(
        &self,
        true_values: std::simd::Simd<T, LANES>,
        false_values: std::simd::Simd<T, LANES>,
    ) -> std::simd::Simd<T, LANES> {
        Self::select(*self, true_values, false_values)
    }
}

/// Broadcast a single element across all components of the vector.
#[inline]
pub fn broadcast<const N: usize, V: Comp<N>>(val: V) -> V::Vec {
    V::new_vec([val; N])
}

/// Creates a new 2D vector from it components.
#[inline]
pub fn vec2<V: Comp<2>>(x: V, y: V) -> V::Vec {
    V::new_vec([x, y])
}

/// Creates a new 3D vector from it components.
#[inline]
pub fn vec3<V: Comp<3>>(x: V, y: V, z: V) -> V::Vec {
    V::new_vec([x, y, z])
}
