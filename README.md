<div align="center">

# ERM

An Educational Ray Marcher written in [Rust](rust-homepage).

</div>

This project is designed to show what a software implementation of a Ray Marcher in pure Rust could look like.
It is distributed under the WTFPL license, so feel free to have a play around!

## Aims

1. To be as fast as possible without sacrificing image quality.
2. Must work with both scalar and SIMD vectors.
3. Core algorithms, like the marching itself, should be understandable.
4. To use a minimal amount of dependencies.

The project makes heavy use of the awesome [`rayon`](rayon-github) library for multithreading and the nightly [`std::simd`](simd-module) for SIMD types.

It also implements it's own Vector types (for now just Vec2, Vec3 and their SIMD counterparts). I wouldn't recommend trying to read the vector module but instead focus on how the it's used in other places in the project; a good start would be `ray.rs`.

## Compiling

To compile the project from source, `git` and `cargo` are required.

If you don't have `cargo` - the Rust package manager - already installed,
follow the steps [here](cargo-install).

Then run
```console
git clone https://github.com/joe-loach/erm
cd erm
cargo build --release
```

> **Note**: Flags passed to rustc include `--target-cpu=native`. This is to include as many features as possible for the current machine.

## Homework

Here are some things to try:

* Removing the `.widen()` from the input arguments to the `render` function, it's magic.

* Modify the scene for a different image.
  > Hint: look at `sdf.rs` for what you can do and could add on.

* Colors are cool, wonder if we could have more than one!
  > Hint: An ID for each static shape might work?

* Not all shapes are static. They should be able to move or appear under certain conditions.
  I wonder if it's possible to have a moving or animating shape?

* Adding a 4 Lane SIMD family of vectors, this would include more hardware as it's supported by SSE.

Most importantly, have some fun with the project. Experiment and make it your own!

[rust-homepage]: https://www.rust-lang.org/
[cargo-install]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[rayon-github]: https://github.com/rayon-rs/rayon
[simd-module]: https://doc.rust-lang.org/nightly/std/simd/index.html