//! This project is used for explaining the DTFSE operation. Here, we have a
//! periodic square signal. The complex form of this signal is represented with
//! s_complex array. DTFSE coefficients are calculated, then, the signal is
//! approximated with the DTFSE function. This function returns its output in
//! real form because original signal has only real parts in this example. The
//! result is kept in the y_real array.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_8_dtfse_calculations`

use core::f32::consts::PI;
use lab4::{display, Shape};
use microfft::Complex32;

use microfft::complex::cfft_16 as cfft;
const N: usize = 16;

fn main() {
    //square signal
    let square = (0..N).map(|idx| if idx < N / 2 { 1.0 } else { 0.0 });
    display("square", Shape::Line, square.clone());

    //map it to real, leave im blank well fill in with cfft
    let mut dtfsecoef: heapless::Vec<Complex32, N> =
        square.map(|f| Complex32 { re: f, im: 0.0 }).collect();

    // SAFETY microfft now only accepts arrays instead of slices to avoid runtime errors
    // Thats not great for us. However we can cheat since our slice into an array because
    // "The layout of a slice [T] of length N is the same as that of a [T; N] array."
    // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
    // this goes away when something like heapless vec is in standard library
    // https://github.com/rust-lang/rfcs/pull/2990
    unsafe {
        let ptr = &mut *(dtfsecoef.as_mut_ptr() as *mut [Complex32; N]);

        // Coefficient calculation with CFFT function
        // well use microfft uses an in place Radix-2 FFT
        // it re-returns our array in case we were going to chain calls, throw it away
        let _ = cfft(ptr);
    }

    println!("dtfsecoef: {:?}", &dtfsecoef);

    //dtfse to reclaim our original signal, note this is a bad approximation for our square wave
    let y_real: heapless::Vec<f32, N> = dtfse(dtfsecoef.iter().cloned(), 1).collect();
    display("y_real 1", Shape::Line, y_real.iter().cloned());

    //a bit better
    let y_real: heapless::Vec<f32, N> = dtfse(dtfsecoef.iter().cloned(), 5).collect();
    display("y_real 5", Shape::Line, y_real.iter().cloned());

    //good
    let y_real: heapless::Vec<f32, N> = dtfse(dtfsecoef.iter().cloned(), 15).collect();
    display("y_real 15", Shape::Line, y_real.iter().cloned());
}

fn dtfse<I: Iterator<Item = Complex32> + Clone>(
    coeff: I,
    k_var: usize,
) -> impl Iterator<Item = f32> {
    let size = N as f32;
    (0..N).map(move |n| {
        coeff
            .clone()
            .take(k_var + 1)
            .enumerate()
            .map(|(k, complex)| {
                let a = (complex.re * complex.re + complex.im * complex.im).sqrt();
                let p = complex.im.atan2(complex.re);
                a * ((2.0 * PI * k as f32 * n as f32 / size) + p).cos() / size
            })
            .sum::<f32>()
    })
}
