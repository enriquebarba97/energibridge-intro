use std::time::Instant;

/// Maximum iterations before we declare a point "in the set".
const MAX_ITER: u32 = 10_000;

/// Image resolution (WIDTH × HEIGHT pixels).
const WIDTH: usize = 4_000;
const HEIGHT: usize = 4_000;

/// Compute how many iterations it takes for the point (cx, cy) to escape.
#[inline]
fn mandelbrot(cx: f64, cy: f64) -> u32 {
    let mut zx = 0.0_f64;
    let mut zy = 0.0_f64;
    for i in 0..MAX_ITER {
        let zx2 = zx * zx;
        let zy2 = zy * zy;
        if zx2 + zy2 > 4.0 {
            return i;
        }
        zy = 2.0 * zx * zy + cy;
        zx = zx2 - zy2 + cx;
    }
    MAX_ITER
}

fn main() {
    println!("=== Sequential Mandelbrot ===");
    println!("Resolution: {WIDTH}×{HEIGHT}, max iterations: {MAX_ITER}\n");

    // We focus on a region near the boundary where computation is expensive.
    let x_min = -0.75;
    let x_max = -0.74;
    let y_min = 0.09;
    let y_max = 0.11;

    let start = Instant::now();

    let mut total_iters: u64 = 0;
    for row in 0..HEIGHT {
        let cy = y_min + (row as f64 / HEIGHT as f64) * (y_max - y_min);
        for col in 0..WIDTH {
            let cx = x_min + (col as f64 / WIDTH as f64) * (x_max - x_min);
            total_iters += mandelbrot(cx, cy) as u64;
        }
    }

    let elapsed = start.elapsed();

    println!("Total iterations computed: {total_iters}");
    println!("Pixels computed: {}", WIDTH * HEIGHT);
    println!("Elapsed: {elapsed:.2?}");
}
