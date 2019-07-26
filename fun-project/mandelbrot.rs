// modified version of original https://github.com/PonasKovas/mandelbrot/blob/master/src/main.rs

extern crate image;
extern crate rayon;
extern crate num;

use rayon::prelude::*;

macro_rules! max_float {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max_float!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min_float {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min_float!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

fn main() {

    // the return of parse() is Err
    let width: u32 = 1080;
    let height = (width as f32 * 9.0 / 16.0) as u32;

    let iterations_per_pixel = (width as f64 / 25_f64) as u32;

    // create a new image for saving
    let img = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(width, height);

    // Distribute the work amongst the cores
    // into_vec : Consumes the image buffer and returns the underlying data as an owned buffer
    // so we need to clone() here
    let mut buffer = img.clone().into_vec();

    // Split the image into rows
    // the par_chunks_mut():
    // Returns a parallel iterator over at most size elements of self at a time.
    // The chunks are mutable and do not overlap.
    buffer.par_chunks_mut(width as usize * 3_usize)
        // change a vec into an iter
        .enumerate()
        // for_each(): Calls a closure on each element of an iterator.
        // which is very similar to the js arrow funciton
        .for_each(
            |(y, mut row)| {
                // Iterate through all pixels in this row
                for x in 0..width {
                    let x = x as usize;
                    // Check if it's in the mandelbrot set
                    match is_point_in_set(x as u32, y, iterations_per_pixel, width, height) {
                        Some(itr) => {

                            // calculate the rgb value
                            let mut r_val = (itr as f32 / iterations_per_pixel as f32) * 255_f32 * 2_f32;
                            let mut gb_val = (1_f32 - (itr as f32 / iterations_per_pixel as f32)) * -255_f32 + 255_f32;

                            r_val = min_float!(r_val, 255_f32);
                            gb_val = max_float!(gb_val, 0_f32);

                            row[x * 3_usize] = r_val as u8;
                            row[x * 3_usize + 1_usize] = gb_val as u8;
                            row[x * 3_usize + 2_usize] = gb_val as u8;
                        }
                        None => {
                            row[x * 3_usize] = 255_u8;
                            row[x * 3_usize + 1_usize] = 255_u8;
                            row[x * 3_usize + 2_usize] = 255_u8;
                        }
                    }
                }
            });

    // Save the outcome
    let final_image: image::RgbImage = image::ImageBuffer::from_vec(width, height, buffer).unwrap();
    final_image.save("mandelbrot.png").unwrap();
}

fn is_point_in_set(x: u32, y: usize, iterations: u32, width: u32, height: u32) -> Option<u32> {
    let c = num::complex::Complex::new(
        (x as f64 / (width - 1) as f64) * 3.5_f64 - 2.5_f64,
        (y as f64 / (height - 1) as f64) * 1.96875_f64 - 0.984375_f64);
    let mut z = num::complex::Complex::new(0_f64, 0_f64);
    for i in 0..iterations {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}