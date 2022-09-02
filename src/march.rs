use crate::ray::Ray;
use crate::vector::prelude::*;
use crate::Vec3;

/// The resulting trace of ray maching.
pub struct Trace<D, H> {
    /// The distance to the surface.
    pub distance: D,
    /// Indicates if the surface was hit by the ray.
    pub hit: H,
}

/// The "precision" of all the algorithms in this module.
pub const EPSILON: f32 = 0.001;
/// The maximum number of steps that the ray marcher can take.
pub const MAX_STEPS: u32 = 64;
/// The maximum distance the ray can travel.
pub const MAX_DIST: f32 = f32::MAX;

/// Returns a [`Trace`] of the ray marching algorithm.
///
/// # Parameters
/// * `map`:    For a given point, returns the maximum distance the marcher can step.
/// * `ray`:    The ray to march, will march from it's origin along it's direction.
/// * `w`:      Step reduction size, can be modified on a per scene basis for improved performance.
///             Default value is `0.87`.
#[inline]
pub fn trace<S, V>(map: &S, ray: Ray<V>, w: Option<V>) -> Trace<V, V::Mask>
where
    V: Traceable,
    S: Fn(V::Vec) -> V,
{
    Traceable::trace(map, ray, w)
}

/// A type that can be used to march a map.
pub trait Traceable: Comp<3> {
    type Mask: Maskable<Self>;

    #[doc(hidden)]
    fn trace<S>(map: &S, ray: Ray<Self>, w: Option<Self>) -> Trace<Self, Self::Mask>
    where
        S: Fn(Self::Vec) -> Self;
}

impl Traceable for f32 {
    type Mask = bool;

    #[inline]
    fn trace<S>(map: &S, ray: Ray<f32>, w: Option<f32>) -> Trace<f32, bool>
    where
        S: Fn(Vec3) -> f32,
    {
        // Implemented using the enhanced sphere tracing algorithm outlined in the paper below.
        // https://diglib.eg.org/bitstream/handle/10.2312/egs20181037/029-032.pdf
        let w = w.unwrap_or(0.87);

        let mut rp = 0.0; // prev
        let mut rc = 1.0; // curr
        let mut rn; // next

        let mut di = 0.0; // overstep distance
        let mut t = 0.0; // total distance

        for _ in 0..MAX_STEPS {
            di = rc + w * rc * ((di - rp + rc) / (di + rp - rc)).max(0.6);
            rn = map(ray.at(t + di));
            if di > rc + rn {
                // fallback to standard sphere tracing algorithm
                di = rc;
                rn = map(ray.at(t + di));
            }
            t += di;
            // (f < (t) * cam_pixel_growth*0.01)
            if rn < t * EPSILON {
                // hit
                return Trace {
                    distance: t,
                    hit: true,
                };
            }
            rp = rc;
            rc = rn;
        }

        Trace {
            distance: MAX_DIST,
            hit: false,
        }
    }
}

use std::simd::{f32x8, mask32x8, SimdPartialOrd};

impl Traceable for f32x8 {
    type Mask = mask32x8;

    #[inline]
    fn trace<S>(map: &S, ray: Ray<f32x8>, _w: Option<f32x8>) -> Trace<f32x8, mask32x8>
    where
        S: Fn(Vec3x8) -> f32x8,
    {
        // use a much simpler version of the ray marching algorithm
        // if the enhanced version above was translated into "simd friendly" code
        // the 'map' function would have to be called twice as no individual branches can be made
        let mut t = f32x8::splat(0.0);
        let mut hit = mask32x8::splat(false);
        for _ in 0..MAX_STEPS {
            // see how far we need to step
            let h = map(ray.at(t));
            // we hit something if the step distance was small
            hit = h.simd_lt(f32x8::splat(EPSILON) * t);
            // rays are finished if they hit something or went too far
            let finished = hit | t.simd_gt(f32x8::splat(MAX_DIST));
            // exit when all rays in the wave are finished
            if finished.all() {
                break;
            }
            // add the step dist to unfinished rays
            t += finished.select(f32x8::splat(0.0), h);
        }
        Trace { distance: t, hit }
    }
}

/// Calculates the surface normal at point `p`.
///
/// The normal is only correct when `p` is on,
/// or very close to, a surface for the given `map`.
pub fn normal<V: Comp<3>, S>(map: &S, p: V::Vec) -> V::Vec
where
    S: Fn(V::Vec) -> V,
{
    // https://iquilezles.org/articles/normalsSDF/
    let x = V::from(1.0);
    let y = V::from(-1.0);
    let ep = V::from(EPSILON);

    let xyy = vec3(x, y, y);
    let yyx = vec3(y, y, x);
    let yxy = vec3(y, x, y);
    let xxx = vec3(x, x, x);
    (xyy * map(p + xyy * ep)
        + yyx * map(p + yyx * ep)
        + yxy * map(p + yxy * ep)
        + xxx * map(p + xxx * ep))
    .normalise()
}
