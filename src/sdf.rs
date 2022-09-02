#![allow(dead_code)]

use crate::vector::prelude::*;

/// Write `use sdf::prelude::*` to easily import useful traits.
pub mod prelude {
    pub use super::Sdf;
    pub use super::SdfExt;
}

/// An object that has a Signed Distance Function.
pub trait Sdf<V: Comp<3>> {
    /// The distance of the surface from point `p` in 3D space.
    fn dist(&self, p: V::Vec) -> V;
}

/// Extensions for modifiying Signed Distance Fields.
pub trait SdfExt<V: Comp<3>>: Sdf<V> + Sized {
    /// Translates the Sdf by `pos`.
    #[inline]
    fn translate(self, pos: V::Vec) -> Translate<V, Self> {
        Translate(self, pos)
    }

    /// Combines two Sdf together.
    #[inline]
    fn union<U: Sdf<V>>(self, other: U) -> Union<V, Self, U> {
        Union::new(self, other)
    }
}

impl<V: Comp<3>, T: Sdf<V> + Sized> SdfExt<V> for T {}

pub use combos::*;
pub use shapes::*;

/// Static shapes and their SDF functions.
mod shapes {
    use std::simd::f32x8;

    use super::*;

    /// A [`Sphere`] with a given radius.
    pub struct Sphere<V: Comp<3>>(pub V);

    impl Sphere<f32> {
        pub fn widen(self) -> Sphere<f32x8> {
            Sphere(f32x8::splat(self.0))
        }
    }

    impl<V: Comp<3>> Sdf<V> for Sphere<V> {
        #[inline]
        fn dist(&self, p: V::Vec) -> V {
            p.length() - self.0
        }
    }

    /// A [`Box`] with radii for each dimension.
    pub struct Box<V: Comp<3>>(pub V::Vec);

    impl Box<f32> {
        pub fn widen(self) -> Box<f32x8> {
            Box(self.0.widen())
        }
    }

    impl<V: Comp<3>> Sdf<V> for Box<V> {
        #[inline]
        fn dist(&self, p: V::Vec) -> V {
            let q = p.abs() - self.0;
            q.max(V::Vec::ZERO).length() + q.max_element().min(V::ZERO)
        }
    }
}

/// Combinations for static shapes.
mod combos {
    use super::*;

    /// Translates an Sdf by the given vector.
    pub struct Translate<V: Comp<3>, S: Sdf<V>>(pub(super) S, pub(super) V::Vec);

    impl<V: Comp<3>, S: Sdf<V>> Sdf<V> for Translate<V, S> {
        #[inline]
        fn dist(&self, p: V::Vec) -> V {
            // shift the input vector by the given vector.
            self.0.dist(p - self.1)
        }
    }

    /// The union of two Sdfs.
    pub struct Union<V: Comp<3>, S: Sdf<V>, U: Sdf<V>> {
        a: S,
        b: U,
        // we don't own a V, it's just for type checking
        _v: core::marker::PhantomData<*const V>,
    }

    impl<V: Comp<3>, S: Sdf<V>, U: Sdf<V>> Union<V, S, U> {
        pub(super) fn new(a: S, b: U) -> Self {
            Self {
                a,
                b,
                _v: core::marker::PhantomData,
            }
        }
    }

    impl<V: Comp<3>, S: Sdf<V>, U: Sdf<V>> Sdf<V> for Union<V, S, U> {
        #[inline]
        fn dist(&self, p: V::Vec) -> V {
            // return the value of the cloesest sdf.
            self.a.dist(p).min(self.b.dist(p))
        }
    }
}
