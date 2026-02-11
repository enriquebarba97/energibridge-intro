# Mandelbrot set computation

This is a simple example on how to make measurements with EnergiBridge to measure the energy consumption of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) computation. To run this code, simply compile it:

```sh
cargo build --release
```

And run the sequential or parallel example through EnergiBridge:

```sh
$PATH_TO_ENERGIBRIDGE [-options] ./target/release/mandelbrot_sequential[.exe]
$PATH_TO_ENERGIBRIDGE [-options] ./target/release/mandelbrot_parallel[.exe]
```