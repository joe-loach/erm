#![feature(portable_simd)]
#![feature(platform_intrinsics)]

mod march;
mod ray;
mod sdf;
mod vector;

use march::Trace;
use ray::Ray;
use sdf::prelude::*;
use vector::prelude::*;

use rayon::prelude::*;
use std::simd::f32x8;

/// The width of the output image.
///
/// For SIMD, must be aligned the LANE count.
const WIDTH: u32 = round_to_nearest::<{ f32x8::LANES as u32 }>(1920);
/// The height of the output image.
const HEIGHT: u32 = 1080;

fn main() {
    // camera origin
    let origin = vec3(0.0, 0.0, 2.0).widen();
    // sun direction
    let ldir = vec3(1.0, 3.0, 1.0).normalise().widen();

    // a sphere with radius 0.5
    let sphere = sdf::Sphere(0.5).widen();
    // the material of the sphere, just a pink color
    let mat = vec3(0.5, 0.2, 0.5).widen();
    // our object is static (doesn't move),
    // we can just return the distance from it
    let map = |p| sphere.dist(p);

    // measure time taken to render
    let start = std::time::Instant::now();

    let buf = render(WIDTH, HEIGHT, origin, ldir, mat, map);

    // print total rendering time
    eprintln!("rendered in {:#?}", start.elapsed());

    // convert to bytes for the image
    let buf = buf.bytes();
    // save the image to "out.png"
    image::save_buffer(
        "out.png",
        buf.as_slice(),
        WIDTH,
        HEIGHT,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

/// Renders and returns a color for each pixel for a given `width` and `height`.
fn render<V>(
    // width of the image
    width: u32,
    // height of the image
    height: u32,
    // ray origin
    origin: <V as Comp<3>>::Vec,
    // light direction for phong shading
    ldir: <V as Comp<3>>::Vec,
    // color of hit objects
    mat: <V as Comp<3>>::Vec,
    // the scene map,
    // basically a fn(Vec3) -> V,
    // needs to be Sync as its shared across threads
    map: impl (Fn(<V as Comp<3>>::Vec) -> V) + Sync,
) -> Vec<<V as Comp<3>>::Vec>
where
    // V:
    // * is a component of 2D and 3D vectors.
    // * can be used to produce a marching trace
    // * generate positions of it's 2D vector
    V: Comp<2> + Comp<3> + march::Traceable + Positions<Inner = <V as Comp<2>>::Vec>,
    // V's 2D vector implements Vector2D
    <V as Comp<2>>::Vec: vector::Vector2D<V>,
{
    // output resolution
    let res = vec2(V::from(width as f32), V::from(height as f32));

    V::positions(width, height)
        .into_par_iter()
        .map(|pos| {
            // RAY GENERATION

            // calculate the "uv" coordiantes from the position on screen
            let uv = ((pos * V::from(2.0)) - res) / -res.min_element();
            // point the ray along the negative Z axis
            let dir = vec3(uv.x(), uv.y(), V::from(-2.0));
            let ray = Ray::new(origin, dir);

            // RAY MARCHING

            let Trace { distance, hit } = march::trace(&map, ray, None);
            // position of where the ray hit
            let pos = ray.at(distance);
            // the surface normal
            let nor = march::normal(&map, pos);

            // LIGHTING

            // amount of light in from phong shading
            let lin: V = phong(ldir, nor, -ray.dir);
            // light up the object in pink
            let col = mat * lin;
            // if not hit, the color should be black
            let col = col * hit.select(V::ONE, V::ZERO);

            // POST PROCESSING

            // gain correction
            let col =
                (col * V::from(1.8)) / (V::ONE + col.dot(broadcast::<3, V>(V::from(1.0 / 3.0))));
            // gamma correction
            col.powf(V::from(1.0 / 2.2))
        })
        .collect()
}

/// Phong shading.
///
/// https://en.wikipedia.org/wiki/Phong_shading
fn phong<V: Comp<3>>(ldir: V::Vec, nor: V::Vec, eye: V::Vec) -> V {
    // material settings
    let ks = V::from(3.0); // specular
    let kd = V::from(3.0); // diffuse
    let ka = V::from(1.0); // ambient
    let al = V::from(20.0); // shinyness

    let rm = nor * nor.dot(ldir) * V::from(2.0) - ldir;
    ka + (kd * ldir.dot(nor).clamp(V::ZERO, V::ONE)
        + ks * rm.dot(eye).clamp(V::ZERO, V::ONE).powf(al))
}

/// A type that is convertable to a stream of bytes to produce an image.
trait ImageBytes {
    fn bytes(self) -> Vec<u8>;
}

/// converts a floating point value to a unsigned byte.
/// assumes that x ∈ [0, 1], but truncates if outside this range.
/// produces a byte ∈ [0, 255] that represents a single color channel.
fn conv(x: f32) -> u8 {
    (x * u8::MAX as f32) as u8
}

impl ImageBytes for Vec<Vec3x8> {
    fn bytes(self) -> Vec<u8> {
        flatten(
            self.into_par_iter()
                .map(|Vec3x8 { x, y, z }| {
                    // because a SIMD vector has multiple lanes, 24 values are produced.
                    let mut arr = [0_u8; f32x8::LANES * 3];
                    for (i, chunk) in arr.chunks_exact_mut(3).enumerate() {
                        assert_eq!(chunk.len(), 3);
                        chunk[0] = conv(x[i]);
                        chunk[1] = conv(y[i]);
                        chunk[2] = conv(z[i]);
                    }
                    arr
                })
                .collect(),
        )
    }
}

impl ImageBytes for Vec<Vec3> {
    fn bytes(self) -> Vec<u8> {
        flatten(
            self.into_par_iter()
                .map(|Vec3 { x, y, z }| [conv(x), conv(y), conv(z)])
                .collect(),
        )
    }
}

/// Generates screen space positions for all pixels in a given `width` and `height`
trait Positions {
    type Inner;

    fn positions(width: u32, height: u32) -> Vec<Self::Inner>;
}

impl Positions for f32 {
    type Inner = Vec2;

    fn positions(width: u32, height: u32) -> Vec<Self::Inner> {
        let pixels = width * height;
        (0..pixels)
            // convert iterator into a parallel iterator
            .into_par_iter()
            // calculate the position on screen
            .map(|i| {
                let x = (i % width) as f32;
                let y = (i / width) as f32;
                vec2(x, y)
            })
            .collect()
    }
}

impl Positions for f32x8 {
    type Inner = Vec2x8;

    fn positions(width: u32, height: u32) -> Vec<Self::Inner> {
        let pixels = width * height;
        (0..pixels)
            // convert iterator into a parallel iterator
            .into_par_iter()
            // step by SIMD lanes (i += LANES)
            .step_by(f32x8::LANES)
            // calculate the position on screen
            .map(|i| {
                const INC: f32x8 = f32x8::from_array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
                // calculate cartesian coordiantes of the pixel
                let x = (i % width) as f32;
                let y = (i / width) as f32;
                let y = f32x8::splat(y); // all Y values are the same
                let x = f32x8::splat(x) + INC; // INC needs to be added so X is 8 different values
                vec2(x, y)
            })
            .collect()
    }
}

/// Flattens a "Vector of array of T" into a "Vector of T".
///
/// Much faster than calling [`Iterator::flatten`] for this specific case.
///
/// # Example
/// ```
/// let v = vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
/// let v = flatten(v);
/// assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
fn flatten<T, const N: usize>(v: Vec<[T; N]>) -> Vec<T> {
    use core::mem::{align_of, size_of, ManuallyDrop};

    // N != 0, size(T) != 0
    // the adjustments to length and capacity would fail
    assert!(N > 0 && size_of::<T>() != 0);
    // a prerequisite of Vec::from_raw_parts,
    // alignment of both types must be the same
    // this is always true for arrays, but check for sanity
    assert_eq!(align_of::<[T; N]>(), align_of::<T>());

    // cannot drop `v` twice,
    // this would happen at the end of this function's scope
    let mut v = ManuallyDrop::new(v);
    // deconstruct `v`
    let (ptr, len, cap) = (v.as_mut_ptr(), v.len(), v.capacity());
    // SAFETY:
    // * ptr is already allocated as a Vec
    // * [T; N] and T have the same alignment
    // * capacity and length are adjusted accordingly
    unsafe { Vec::from_raw_parts(ptr.cast(), len * N, cap * N) }
}

/// Rounds `x` up to the nearest multiple of the value provided.
const fn round_to_nearest<const M: u32>(x: u32) -> u32 {
    (x + M - 1) & (u32::MAX - M + 1)
}
