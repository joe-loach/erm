use crate::vector::prelude::*;

/// A Ray in 3D space with an `origin` and `direction`.
#[derive(Clone, Copy)]
pub struct Ray<V: Comp<3>> {
    /// The origin point of the ray.
    pub origin: V::Vec,
    /// The direction the ray is pointing.
    pub dir: V::Vec,
}

impl<V: Comp<3>> Ray<V> {
    /// Creates a new [`Ray`] with an `origin` and `direction`.
    pub fn new(origin: V::Vec, dir: V::Vec) -> Self {
        // |dir| == 1, otherwise distance calculations will be incorrect
        let dir = dir.normalise();
        Self { origin, dir }
    }

    /// Returns the point along the [`Ray`] at distance `t`.
    pub fn at(&self, t: V) -> V::Vec {
        // use the mul_add instruction,
        // it's a single instruction so is faster and increases fp accuracy
        broadcast(t).mul_add(self.dir, self.origin)
    }
}
