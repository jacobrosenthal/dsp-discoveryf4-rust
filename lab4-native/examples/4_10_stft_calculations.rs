//! This project is used for explaining the STFT operation. Here we have a chirp
//! signal and Hamming window. These signals are represented with x and v arrays
//! in main.c file respectively. The input signal is divided into subwindows and
//! FFT of each subwindow is calculated by the STFT function. The result is
//! stored in the XST array.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over to no_std without alloc.
//!
//! `cargo run --example 4_10_stft_calculations`

use core::f32::consts::PI;
use lab4::{display, Shape};
use microfft::Complex32;
use plotly::HeatMap;

use microfft::complex::cfft_16 as cfft;
const WINDOW: usize = 16;

const N: usize = 1024;
const NDIV2: usize = N / 2;

const W1: f32 = 0.0;
const W2: f32 = core::f32::consts::PI;

fn main() {
    let chirp: heapless::Vec<f32, N> = (0..N)
        .map(|n| {
            let n = n as f32;
            (W1 * n + (W2 - W1) * n * n / (2.0 * (N as f32 - 1.0))).cos()
        })
        .collect();

    let hamming = (0..WINDOW).map(|m| 0.54 - 0.46 * (2.0 * PI * m as f32 / WINDOW as f32).cos());
    display("hamming", Shape::Line, hamming.clone());

    let overlapping_chirp_windows = Windows {
        v: &chirp,
        size: WINDOW,
        inc: WINDOW / 2,
    };

    let xst: heapless::Vec<_, NDIV2> = overlapping_chirp_windows
        .map(|chirp_win| {
            let mut dtfsecoef: heapless::Vec<Complex32, WINDOW> = hamming
                .clone()
                .zip(chirp_win.iter().rev())
                .map(|(v, x)| Complex32 { re: v * x, im: 0.0 })
                .collect();

            // SAFETY microfft now only accepts arrays instead of slices to avoid runtime errors
            // Thats not great for us. However we can cheat since our slice into an array because
            // "The layout of a slice [T] of length N is the same as that of a [T; N] array."
            // https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html
            // this goes away when something like heapless vec is in standard library
            // https://github.com/rust-lang/rfcs/pull/2990
            unsafe {
                let ptr = &mut *(dtfsecoef.as_mut_ptr() as *mut [Complex32; WINDOW]);

                // Coefficient calculation with CFFT function
                // well use microfft uses an in place Radix-2 FFT
                // it re-returns our array in case we were going to chain calls, throw it away
                let _ = cfft(ptr);
            }

            // Magnitude calculation
            let mag: heapless::Vec<_, WINDOW> = dtfsecoef
                .iter()
                .map(|complex| (complex.re * complex.re + complex.im * complex.im).sqrt())
                .collect();
            mag
        })
        .collect();

    // // the answer key data for M=16
    // let z: Vec<Vec<f32>> = Windows {
    //     v: &ZZ,
    //     size: WINDOW,
    //     inc: WINDOW,
    // }
    // .map(|slice| slice.to_vec())
    // .collect();
    // println!("z:{:?}", z);

    // why are we 127 instead of 126? maybe they off by one errored? definately rounding differences too
    let mut z: std::vec::Vec<Vec<f32>> = xst.iter().map(|v| v.to_vec()).collect();
    z.pop();

    plot(z);
}

fn plot(z: Vec<Vec<f32>>) {
    let z = clean(z);
    let trace = HeatMap::new_z(z);
    let mut plot = plotly::Plot::new();

    plot.add_trace(trace);
    plot.show();
}

// i will never understand..
fn clean(zzzz: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    //any better way to transpose a vec of vecs?
    let mut z = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    // throws away 8-15????
    for v in zzzz {
        z[0].push(0.0); // XST1(1,:)=0;
        z[1].push(v[1]);
        z[2].push(v[2]);
        z[3].push(v[3]);
        z[4].push(v[4]);
        z[5].push(v[5]);
        z[6].push(v[6]);
        z[7].push(0.0); // XST1(end,:)=0;
    }

    // XST1=flipud(XST);
    z.reverse();

    for v in &mut z {
        // XST1(:,1)=0;
        (*v)[0] = 0.0;
        // XST1(:,end)=0;
        let end = (*v).len() - 1;
        (*v)[end] = 0.0;
    }

    // f=surf(fliplr(XST2));
    for v in &mut z {
        (*v).reverse()
    }

    // // XST2=XST1(2:end,2:end);
    // for v in &mut z {
    //     v.pop();
    // }
    // z.drain(0..1);

    z
}

/// copied from std::slice::Window but expose the increment amount instead of using 1
pub struct Windows<'a, T: 'a> {
    v: &'a [T],
    size: usize,
    inc: usize,
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.size > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.size]);
            self.v = &self.v[self.inc..];
            ret
        }
    }
}

#[allow(unused)]
static ZZ: &[f32] = &[
    8.5525, 3.6769, 0.0085, 0.0025, 0.0035, 0.0036, 0.0035, 0.0034, 0.0033, 0.0034, 0.0035, 0.0036,
    0.0035, 0.0025, 0.0085, 3.6769, 7.8146, 3.4681, 0.0339, 0.0118, 0.0164, 0.0165, 0.0159, 0.0155,
    0.0153, 0.0155, 0.0159, 0.0165, 0.0164, 0.0118, 0.0339, 3.4681, 5.1868, 2.6222, 0.0941, 0.0327,
    0.0418, 0.0414, 0.0398, 0.0387, 0.0382, 0.0387, 0.0398, 0.0414, 0.0418, 0.0327, 0.0941, 2.6222,
    0.1890, 1.6524, 0.1829, 0.0556, 0.0633, 0.0618, 0.0592, 0.0573, 0.0567, 0.0573, 0.0592, 0.0618,
    0.0633, 0.0556, 0.1829, 1.6524, 6.2936, 3.1743, 0.2376, 0.0483, 0.0412, 0.0384, 0.0362, 0.0349,
    0.0345, 0.0349, 0.0362, 0.0384, 0.0412, 0.0483, 0.2376, 3.1743, 7.0434, 3.7607, 0.2471, 0.0170,
    0.0393, 0.0413, 0.0403, 0.0393, 0.0389, 0.0393, 0.0403, 0.0413, 0.0393, 0.0170, 0.2471, 3.7607,
    0.9254, 2.7262, 0.4162, 0.0805, 0.0858, 0.0833, 0.0795, 0.0769, 0.0760, 0.0769, 0.0795, 0.0833,
    0.0858, 0.0805, 0.4162, 2.7262, 7.0468, 4.0327, 0.4731, 0.0257, 0.0089, 0.0137, 0.0145, 0.0145,
    0.0144, 0.0145, 0.0145, 0.0137, 0.0089, 0.0257, 0.4731, 4.0327, 0.8448, 3.3108, 0.6348, 0.0840,
    0.0853, 0.0827, 0.0788, 0.0762, 0.0753, 0.0762, 0.0788, 0.0827, 0.0853, 0.0840, 0.6348, 3.3108,
    5.7227, 4.1242, 0.7150, 0.0041, 0.0382, 0.0431, 0.0427, 0.0418, 0.0415, 0.0418, 0.0427, 0.0431,
    0.0382, 0.0041, 0.7150, 4.1242, 4.6283, 3.9896, 0.9342, 0.0764, 0.0412, 0.0352, 0.0323, 0.0307,
    0.0302, 0.0307, 0.0323, 0.0352, 0.0412, 0.0764, 0.9342, 3.9896, 0.1497, 3.9583, 1.0679, 0.0676,
    0.0618, 0.0601, 0.0574, 0.0555, 0.0548, 0.0555, 0.0574, 0.0601, 0.0618, 0.0676, 1.0679, 3.9583,
    3.0589, 4.2296, 1.2192, 0.0283, 0.0397, 0.0422, 0.0411, 0.0400, 0.0396, 0.0400, 0.0411, 0.0422,
    0.0397, 0.0283, 1.2192, 4.2296, 4.1527, 4.3357, 1.4219, 0.0207, 0.0147, 0.0183, 0.0185, 0.0182,
    0.0180, 0.0182, 0.0185, 0.0183, 0.0147, 0.0207, 1.4219, 4.3357, 4.0747, 4.3373, 1.6335, 0.0221,
    0.0025, 0.0049, 0.0053, 0.0053, 0.0053, 0.0053, 0.0053, 0.0049, 0.0025, 0.0221, 1.6335, 4.3373,
    3.6747, 4.3201, 1.8440, 0.0191, 0.0021, 0.0004, 0.0002, 0.0002, 0.0002, 0.0002, 0.0002, 0.0004,
    0.0021, 0.0191, 1.8440, 4.3201, 3.2415, 4.2905, 2.0599, 0.0336, 0.0039, 0.0057, 0.0062, 0.0062,
    0.0062, 0.0062, 0.0062, 0.0057, 0.0039, 0.0336, 2.0599, 4.2905, 2.6431, 4.2440, 2.2885, 0.0533,
    0.0132, 0.0199, 0.0206, 0.0204, 0.0203, 0.0204, 0.0206, 0.0199, 0.0132, 0.0533, 2.2885, 4.2440,
    1.5923, 4.1940, 2.5251, 0.0717, 0.0346, 0.0438, 0.0440, 0.0431, 0.0428, 0.0431, 0.0440, 0.0438,
    0.0346, 0.0717, 2.5251, 4.1940, 0.0414, 4.1405, 2.7294, 0.1423, 0.0564, 0.0595, 0.0577, 0.0560,
    0.0553, 0.0560, 0.0577, 0.0595, 0.0564, 0.1423, 2.7294, 4.1405, 1.3584, 4.0073, 2.9029, 0.2518,
    0.0457, 0.0302, 0.0261, 0.0241, 0.0235, 0.0241, 0.0261, 0.0302, 0.0457, 0.2518, 2.9029, 4.0073,
    1.4233, 3.8080, 3.1704, 0.2748, 0.0214, 0.0483, 0.0517, 0.0516, 0.0513, 0.0516, 0.0517, 0.0483,
    0.0214, 0.2748, 3.1704, 3.8080, 0.0311, 3.7305, 3.3518, 0.3849, 0.0799, 0.0785, 0.0756, 0.0730,
    0.0721, 0.0730, 0.0756, 0.0785, 0.0799, 0.3849, 3.3518, 3.7305, 0.9966, 3.4987, 3.5270, 0.5052,
    0.0174, 0.0229, 0.0292, 0.0302, 0.0303, 0.0302, 0.0292, 0.0229, 0.0174, 0.5052, 3.5270, 3.4987,
    0.0272, 3.3594, 3.7125, 0.6140, 0.0836, 0.0776, 0.0745, 0.0719, 0.0709, 0.0719, 0.0745, 0.0776,
    0.0836, 0.6140, 3.7125, 3.3594, 0.5947, 3.0963, 3.8876, 0.7301, 0.0092, 0.0483, 0.0541, 0.0544,
    0.0542, 0.0544, 0.0541, 0.0483, 0.0092, 0.7301, 3.8876, 3.0963, 0.2778, 2.9551, 3.9635, 0.9391,
    0.0696, 0.0282, 0.0214, 0.0187, 0.0179, 0.0187, 0.0214, 0.0282, 0.0696, 0.9391, 3.9635, 2.9551,
    0.0948, 2.7134, 4.1213, 1.0622, 0.0696, 0.0570, 0.0549, 0.0530, 0.0523, 0.0530, 0.0549, 0.0570,
    0.0696, 1.0622, 4.1213, 2.7134, 0.1947, 2.4637, 4.2290, 1.2224, 0.0353, 0.0424, 0.0446, 0.0442,
    0.0439, 0.0442, 0.0446, 0.0424, 0.0353, 1.2224, 4.2290, 2.4637, 0.1364, 2.2529, 4.2781, 1.4278,
    0.0212, 0.0194, 0.0230, 0.0233, 0.0233, 0.0233, 0.0230, 0.0194, 0.0212, 1.4278, 4.2781, 2.2529,
    0.0569, 2.0455, 4.3073, 1.6378, 0.0210, 0.0057, 0.0080, 0.0083, 0.0084, 0.0083, 0.0080, 0.0057,
    0.0210, 1.6378, 4.3073, 2.0455, 0.0024, 1.8339, 4.3191, 1.8476, 0.0195, 0.0021, 0.0005, 0.0004,
    0.0004, 0.0004, 0.0005, 0.0021, 0.0195, 1.8476, 4.3191, 1.8339, 0.0304, 1.6279, 4.3038, 2.0643,
    0.0314, 0.0061, 0.0091, 0.0097, 0.0098, 0.0097, 0.0091, 0.0061, 0.0314, 2.0643, 4.3038, 1.6279,
    0.0506, 1.4387, 4.2522, 2.2947, 0.0479, 0.0179, 0.0252, 0.0262, 0.0262, 0.0262, 0.0252, 0.0179,
    0.0479, 2.2947, 4.2522, 1.4387, 0.0562, 1.2638, 4.1700, 2.5296, 0.0742, 0.0383, 0.0468, 0.0474,
    0.0473, 0.0474, 0.0468, 0.0383, 0.0742, 2.5296, 4.1700, 1.2638, 0.0338, 1.0683, 4.0998, 2.7274,
    0.1550, 0.0535, 0.0537, 0.0524, 0.0518, 0.0524, 0.0537, 0.0535, 0.1550, 2.7274, 4.0998, 1.0683,
    0.0243, 0.8655, 4.0250, 2.9068, 0.2487, 0.0324, 0.0142, 0.0101, 0.0087, 0.0101, 0.0142, 0.0324,
    0.2487, 2.9068, 4.0250, 0.8655, 0.0838, 0.7778, 3.8230, 3.1806, 0.2645, 0.0354, 0.0613, 0.0649,
    0.0653, 0.0649, 0.0613, 0.0354, 0.2645, 3.1806, 3.8230, 0.7778, 0.0630, 0.6192, 3.6998, 3.3437,
    0.4029, 0.0762, 0.0682, 0.0655, 0.0644, 0.0655, 0.0682, 0.0762, 0.4029, 3.3437, 3.6998, 0.6192,
    0.0484, 0.4921, 3.5210, 3.5425, 0.4882, 0.0025, 0.0418, 0.0482, 0.0491, 0.0482, 0.0418, 0.0025,
    0.4882, 3.5425, 3.5210, 0.4921, 0.0720, 0.3980, 3.3384, 3.7022, 0.6331, 0.0820, 0.0670, 0.0638,
    0.0626, 0.0638, 0.0670, 0.0820, 0.6331, 3.7022, 3.3384, 0.3980, 0.0557, 0.3259, 3.1007, 3.8980,
    0.7217, 0.0277, 0.0616, 0.0673, 0.0679, 0.0673, 0.0616, 0.0277, 0.7217, 3.8980, 3.1007, 0.3259,
    0.0273, 0.1891, 2.9553, 3.9698, 0.9353, 0.0562, 0.0111, 0.0046, 0.0019, 0.0046, 0.0111, 0.0562,
    0.9353, 3.9698, 2.9553, 0.1891, 0.0603, 0.1762, 2.7047, 4.1117, 1.0787, 0.0724, 0.0505, 0.0477,
    0.0469, 0.0477, 0.0505, 0.0724, 1.0787, 4.1117, 2.7047, 0.1762, 0.0433, 0.1339, 2.4604, 4.2265,
    1.2302, 0.0463, 0.0454, 0.0471, 0.0470, 0.0471, 0.0454, 0.0463, 1.2302, 4.2265, 2.4604, 0.1339,
    0.0178, 0.0734, 2.2510, 4.2822, 1.4268, 0.0255, 0.0252, 0.0283, 0.0286, 0.0283, 0.0252, 0.0255,
    1.4268, 4.2822, 2.2510, 0.0734, 0.0041, 0.0327, 2.0427, 4.3106, 1.6375, 0.0206, 0.0095, 0.0114,
    0.0116, 0.0114, 0.0095, 0.0206, 1.6375, 4.3106, 2.0427, 0.0327, 0.0009, 0.0190, 1.8312, 4.3188,
    1.8512, 0.0195, 0.0021, 0.0008, 0.0007, 0.0008, 0.0021, 0.0195, 1.8512, 4.3188, 1.8312, 0.0190,
    0.0009, 0.0258, 1.6269, 4.2995, 2.0718, 0.0272, 0.0094, 0.0132, 0.0137, 0.0132, 0.0094, 0.0272,
    2.0718, 4.2995, 1.6269, 0.0258, 0.0112, 0.0421, 1.4394, 4.2470, 2.3022, 0.0421, 0.0238, 0.0310,
    0.0320, 0.0310, 0.0238, 0.0421, 2.3022, 4.2470, 1.4394, 0.0421, 0.0354, 0.0605, 1.2608, 4.1739,
    2.5268, 0.0828, 0.0425, 0.0493, 0.0499, 0.0493, 0.0425, 0.0828, 2.5268, 4.1739, 1.2608, 0.0605,
    0.0611, 0.0608, 1.0570, 4.1127, 2.7155, 0.1698, 0.0488, 0.0452, 0.0444, 0.0452, 0.0488, 0.1698,
    2.7155, 4.1127, 1.0570, 0.0608, 0.0488, 0.0157, 0.8657, 4.0141, 2.9205, 0.2395, 0.0158, 0.0065,
    0.0095, 0.0065, 0.0158, 0.2395, 2.9205, 4.0141, 0.8657, 0.0157, 0.0274, 0.0638, 0.7841, 3.8118,
    3.1920, 0.2583, 0.0524, 0.0747, 0.0778, 0.0747, 0.0524, 0.2583, 3.1920, 3.8118, 0.7841, 0.0638,
    0.0871, 0.0791, 0.6022, 3.7158, 3.3277, 0.4211, 0.0698, 0.0536, 0.0513, 0.0536, 0.0698, 0.4211,
    3.3277, 3.7158, 0.6022, 0.0791, 0.0046, 0.0289, 0.5069, 3.4969, 3.5667, 0.4685, 0.0263, 0.0628,
    0.0679, 0.0628, 0.0263, 0.4685, 3.5667, 3.4969, 0.5069, 0.0289, 0.0878, 0.0801, 0.3794, 3.3539,
    3.6861, 0.6518, 0.0773, 0.0520, 0.0490, 0.0520, 0.0773, 0.6518, 3.6861, 3.3539, 0.3794, 0.0801,
    0.0284, 0.0483, 0.3344, 3.0899, 3.9067, 0.7188, 0.0494, 0.0749, 0.0790, 0.0749, 0.0494, 0.7188,
    3.9067, 3.0899, 0.3344, 0.0483, 0.0496, 0.0327, 0.1925, 2.9401, 3.9851, 0.9239, 0.0381, 0.0101,
    0.0163, 0.0101, 0.0381, 0.9239, 3.9851, 2.9401, 0.1925, 0.0327, 0.0633, 0.0606, 0.1613, 2.7163,
    4.0984, 1.0952, 0.0725, 0.0408, 0.0377, 0.0408, 0.0725, 1.0952, 4.0984, 2.7163, 0.1613, 0.0606,
    0.0376, 0.0424, 0.1313, 2.4656, 4.2182, 1.2432, 0.0574, 0.0470, 0.0474, 0.0470, 0.0574, 1.2432,
    4.2182, 2.4656, 0.1313, 0.0424, 0.0125, 0.0179, 0.0782, 2.2452, 4.2847, 1.4288, 0.0341, 0.0307,
    0.0325, 0.0307, 0.0341, 1.4288, 4.2847, 2.2452, 0.0782, 0.0179, 0.0015, 0.0048, 0.0363, 2.0361,
    4.3140, 1.6379, 0.0230, 0.0133, 0.0144, 0.0133, 0.0230, 1.6379, 4.3140, 2.0361, 0.0363, 0.0048,
    0.0001, 0.0019, 0.0191, 1.8283, 4.3183, 1.8549, 0.0192, 0.0018, 0.0009, 0.0018, 0.0192, 1.8549,
    4.3183, 1.8283, 0.0191, 0.0019, 0.0023, 0.0042, 0.0277, 1.6278, 4.2955, 2.0786, 0.0221, 0.0132,
    0.0165, 0.0132, 0.0221, 2.0786, 4.2955, 1.6278, 0.0277, 0.0042, 0.0149, 0.0168, 0.0442, 1.4387,
    4.2452, 2.3060, 0.0412, 0.0295, 0.0348, 0.0295, 0.0412, 2.3060, 4.2452, 1.4387, 0.0442, 0.0168,
    0.0413, 0.0415, 0.0570, 1.2476, 4.1847, 2.5179, 0.0978, 0.0449, 0.0481, 0.0449, 0.0978, 2.5179,
    4.1847, 1.2476, 0.0570, 0.0415, 0.0646, 0.0612, 0.0460, 1.0383, 4.1249, 2.7056, 0.1814, 0.0408,
    0.0334, 0.0408, 0.1814, 2.7056, 4.1249, 1.0383, 0.0460, 0.0612, 0.0430, 0.0368, 0.0120, 0.8799,
    3.9934, 2.9434, 0.2216, 0.0083, 0.0254, 0.0083, 0.2216, 2.9434, 3.9934, 0.8799, 0.0120, 0.0368,
    0.0395, 0.0430, 0.0753, 0.7859, 3.8103, 3.1930, 0.2651, 0.0683, 0.0820, 0.0683, 0.2651, 3.1930,
    3.8103, 0.7859, 0.0753, 0.0430, 0.0863, 0.0826, 0.0603, 0.5807, 3.7266, 3.3185, 0.4319, 0.0582,
    0.0362, 0.0582, 0.4319, 3.3185, 3.7266, 0.5807, 0.0603, 0.0826, 0.0119, 0.0167, 0.0514, 0.5266,
    3.4762, 3.5856, 0.4553, 0.0514, 0.0766, 0.0514, 0.4553, 3.5856, 3.4762, 0.5266, 0.0514, 0.0167,
    0.0845, 0.0817, 0.0630, 0.3575, 3.3630, 3.6779, 0.6619, 0.0666, 0.0353, 0.0666, 0.6619, 3.6779,
    3.3630, 0.3575, 0.0630, 0.0817, 0.0436, 0.0461, 0.0636, 0.3373, 3.0905, 3.9034, 0.7293, 0.0701,
    0.0800, 0.0701, 0.7293, 3.9034, 3.0905, 0.3373, 0.0636, 0.0461, 0.0348, 0.0316, 0.0099, 0.2091,
    2.9156, 4.0090, 0.9040, 0.0158, 0.0254, 0.0158, 0.9040, 4.0090, 2.9156, 0.2091, 0.0099, 0.0316,
    0.0601, 0.0588, 0.0509, 0.1418, 2.7235, 4.0906, 1.1060, 0.0671, 0.0310, 0.0671, 1.1060, 4.0906,
    2.7235, 0.1418, 0.0509, 0.0588, 0.0425, 0.0432, 0.0456, 0.1224, 2.4768, 4.2045, 1.2614, 0.0655,
    0.0453, 0.0655, 1.2614, 4.2045, 2.4768, 0.1224, 0.0456, 0.0432, 0.0191, 0.0200, 0.0253, 0.0814,
    2.2445, 4.2817, 1.4364, 0.0437, 0.0327, 0.0437, 1.4364, 4.2817, 2.2445, 0.0814, 0.0253, 0.0200,
    0.0054, 0.0060, 0.0097, 0.0406, 2.0312, 4.3153, 1.6405, 0.0274, 0.0154, 0.0274, 1.6405, 4.3153,
    2.0312, 0.0406, 0.0097, 0.0060, 0.0009, 0.0006, 0.0016, 0.0186, 1.8254, 4.3180, 1.8584, 0.0186,
    0.0011, 0.0186, 1.8584, 4.3180, 1.8254, 0.0186, 0.0016, 0.0006, 0.0078, 0.0082, 0.0090, 0.0275,
    1.6263, 4.2940, 2.0824, 0.0190, 0.0120, 0.0190, 2.0824, 4.2940, 1.6263, 0.0275, 0.0090, 0.0082,
    0.0237, 0.0241, 0.0251, 0.0428, 1.4316, 4.2500, 2.3033, 0.0490, 0.0269, 0.0490, 2.3033, 4.2500,
    1.4316, 0.0428, 0.0251, 0.0241, 0.0473, 0.0475, 0.0463, 0.0475, 1.2280, 4.2007, 2.5042, 0.1133,
    0.0396, 0.1133, 2.5042, 4.2007, 1.2280, 0.0475, 0.0463, 0.0475, 0.0579, 0.0573, 0.0521, 0.0244,
    1.0271, 4.1280, 2.7057, 0.1835, 0.0349, 0.1835, 2.7057, 4.1280, 1.0271, 0.0244, 0.0521, 0.0573,
    0.0198, 0.0187, 0.0112, 0.0376, 0.9030, 3.9655, 2.9722, 0.2005, 0.0040, 0.2005, 2.9722, 3.9655,
    0.9030, 0.0376, 0.0112, 0.0187, 0.0590, 0.0602, 0.0634, 0.0818, 0.7723, 3.8253, 3.1779, 0.2861,
    0.0605, 0.2861, 3.1779, 3.8253, 0.7723, 0.0818, 0.0634, 0.0602, 0.0717, 0.0712, 0.0657, 0.0326,
    0.5740, 3.7213, 3.3264, 0.4272, 0.0597, 0.4272, 3.3264, 3.7213, 0.5740, 0.0326, 0.0657, 0.0712,
    0.0396, 0.0412, 0.0465, 0.0728, 0.5348, 3.4713, 3.5884, 0.4604, 0.0368, 0.4604, 3.5884, 3.4713,
    0.5348, 0.0728, 0.0465, 0.0412, 0.0685, 0.0680, 0.0639, 0.0370, 0.3512, 3.3549, 3.6877, 0.6562,
    0.0773, 0.6562, 3.6877, 3.3549, 0.3512, 0.0370, 0.0639, 0.0680, 0.0620, 0.0629, 0.0660, 0.0749,
    0.3250, 3.1083, 3.8841, 0.7501, 0.0555, 0.7501, 3.8841, 3.1083, 0.3250, 0.0749, 0.0660, 0.0629,
    0.0091, 0.0087, 0.0058, 0.0185, 0.2340, 2.8865, 4.0351, 0.8892, 0.0434, 0.8892, 4.0351, 2.8865,
    0.2340, 0.0185, 0.0058, 0.0087, 0.0487, 0.0486, 0.0469, 0.0345, 0.1309, 2.7184, 4.0957, 1.1051,
    0.0893, 1.1051, 4.0957, 2.7184, 0.1309, 0.0345, 0.0469, 0.0486, 0.0449, 0.0452, 0.0459, 0.0446,
    0.1059, 2.4899, 4.1901, 1.2742, 0.0706, 1.2742, 4.1901, 2.4899, 0.1059, 0.0446, 0.0459, 0.0452,
    0.0257, 0.0261, 0.0275, 0.0307, 0.0790, 2.2507, 4.2729, 1.4454, 0.0371, 1.4454, 4.2729, 2.2507,
    0.0790, 0.0307, 0.0275, 0.0261, 0.0096, 0.0098, 0.0109, 0.0140, 0.0431, 2.0301, 4.3133, 1.6441,
    0.0219, 1.6441, 4.3133, 2.0301, 0.0431, 0.0140, 0.0109, 0.0098, 0.0017, 0.0015, 0.0013, 0.0010,
    0.0177, 1.8220, 4.3184, 1.8597, 0.0309, 1.8597, 4.3184, 1.8220, 0.0177, 0.0010, 0.0013, 0.0015,
    0.0136, 0.0138, 0.0142, 0.0140, 0.0242, 1.6201, 4.2975, 2.0817, 0.0508, 2.0817, 4.2975, 1.6201,
    0.0242, 0.0140, 0.0142, 0.0138, 0.0309, 0.0312, 0.0320, 0.0315, 0.0361, 1.4168, 4.2610, 2.3003,
    0.0479, 2.3003, 4.2610, 1.4168, 0.0361, 0.0315, 0.0320, 0.0312, 0.0478, 0.0481, 0.0486, 0.0460,
    0.0309, 1.2079, 4.2132, 2.5055, 0.0386, 2.5055, 4.2132, 1.2079, 0.0309, 0.0460, 0.0486, 0.0481,
    0.0420, 0.0419, 0.0412, 0.0352, 0.0118, 1.0326, 4.1155, 2.7167, 0.2489, 2.7167, 4.1155, 1.0326,
    0.0118, 0.0352, 0.0412, 0.0419, 0.0098, 0.0107, 0.0133, 0.0199, 0.0632, 0.9246, 3.9447, 2.9742,
    0.4215, 2.9742, 3.9447, 0.9246, 0.0632, 0.0199, 0.0133, 0.0107, 0.0731, 0.0738, 0.0762, 0.0776,
    0.0783, 0.7415, 3.8507, 3.1791, 0.1040, 3.1791, 3.8507, 0.7415, 0.0783, 0.0776, 0.0762, 0.0738,
    0.0448, 0.0451, 0.0443, 0.0381, 0.0047, 0.5919, 3.6971, 3.3312, 0.7082, 3.3312, 3.6971, 0.5919,
    0.0047, 0.0381, 0.0443, 0.0451, 0.0649, 0.0659, 0.0688, 0.0732, 0.0867, 0.5206, 3.4870, 3.5914,
    0.5053, 3.5914, 3.4870, 0.5206, 0.0867, 0.0732, 0.0688, 0.0659, 0.0405, 0.0404, 0.0400, 0.0351,
    0.0025, 0.3707, 3.3298, 3.6826, 1.1441, 3.6826, 3.3298, 0.3707, 0.0025, 0.0351, 0.0400, 0.0404,
    0.0729, 0.0738, 0.0761, 0.0784, 0.0767, 0.2957, 3.1305, 3.9061, 0.1529, 3.9061, 3.1305, 0.2957,
    0.0767, 0.0784, 0.0761, 0.0738, 0.0204, 0.0211, 0.0231, 0.0277, 0.0464, 0.2551, 2.8712, 4.0211,
    1.6793, 4.0211, 2.8712, 0.2551, 0.0464, 0.0277, 0.0231, 0.0211, 0.0289, 0.0288, 0.0289, 0.0266,
    0.0111, 0.1403, 2.7017, 4.0772, 1.9415, 4.0772, 2.7017, 0.1403, 0.0111, 0.0266, 0.0289, 0.0288,
    0.0400, 0.0407, 0.0413, 0.0419, 0.0364, 0.0876, 2.4933, 4.2021, 1.0881, 4.2021, 2.4933, 0.0876,
    0.0364, 0.0419, 0.0413, 0.0407, 0.0287, 0.0289, 0.0301, 0.0311, 0.0322, 0.0694, 2.2557, 4.2967,
    0.0723, 4.2967, 2.2557, 0.0694, 0.0322, 0.0311, 0.0301, 0.0289, 0.0124, 0.0128, 0.0131, 0.0143,
    0.0163, 0.0424, 2.0294, 4.3332, 0.9744, 4.3332, 2.0294, 0.0424, 0.0163, 0.0143, 0.0131, 0.0128,
    0.0025, 0.0022, 0.0024, 0.0021, 0.0003, 0.0170, 1.8169, 4.3295, 1.3690, 4.3295, 1.8169, 0.0170,
    0.0003, 0.0021, 0.0024, 0.0022, 0.0176, 0.0176, 0.0182, 0.0187, 0.0174, 0.0176, 1.6118, 4.2856,
    1.0549, 4.2856, 1.6118, 0.0176, 0.0174, 0.0187, 0.0182, 0.0176, 0.0330, 0.0337, 0.0343, 0.0355,
    0.0334, 0.0234, 1.4078, 4.1995, 0.2780, 4.1995, 1.4078, 0.0234, 0.0334, 0.0355, 0.0343, 0.0337,
    0.0397, 0.0401, 0.0411, 0.0416, 0.0377, 0.0115, 1.2065, 4.1467, 2.7399, 4.1467, 1.2065, 0.0115,
    0.0377, 0.0416, 0.0411, 0.0401, 0.0169, 0.0170, 0.0170, 0.0162, 0.0101, 0.0358, 1.0399, 4.2272,
    5.2623, 4.2272, 1.0399, 0.0358, 0.0101, 0.0162, 0.0170, 0.0170, 0.0397, 0.0404, 0.0422, 0.0451,
    0.0502, 0.0784, 0.9187, 4.0475, 4.8133, 4.0475, 0.9187, 0.0784, 0.0502, 0.0451, 0.0422, 0.0404,
    0.0741, 0.0751, 0.0772, 0.0802, 0.0795, 0.0613, 0.7368, 3.5567, 0.9738, 3.5567, 0.7368, 0.0613,
    0.0795, 0.0802, 0.0772, 0.0751, 0.0077, 0.0079, 0.0077, 0.0064, 0.0007, 0.0375, 0.5911, 4.0680,
    6.6901, 4.0680, 0.5911, 0.0375, 0.0007, 0.0064, 0.0077, 0.0079, 0.0790, 0.0801, 0.0825, 0.0867,
    0.0890, 0.0847, 0.5209, 3.0374, 1.3371, 3.0374, 0.5209, 0.0847, 0.0890, 0.0867, 0.0825, 0.0801,
    0.0027, 0.0027, 0.0028, 0.0021, 0.0032, 0.0296, 0.3591, 3.9381, 7.4353, 3.9381, 0.3591, 0.0296,
    0.0032, 0.0021, 0.0028, 0.0027, 0.0700, 0.0706, 0.0728, 0.0762, 0.0770, 0.0654, 0.3111, 2.4786,
    1.9499, 2.4786, 0.3111, 0.0654, 0.0770, 0.0762, 0.0728, 0.0706, 0.0474, 0.0481, 0.0498, 0.0526,
    0.0562, 0.0629, 0.2503, 3.0686, 5.7581, 3.0686, 0.2503, 0.0629, 0.0562, 0.0526, 0.0498, 0.0481,
    0.0018, 0.0019, 0.0019, 0.0015, 0.0005, 0.0101, 0.1157, 3.7576, 8.2161, 3.7576, 0.1157, 0.0101,
    0.0005, 0.0015, 0.0019, 0.0019, 0.0267, 0.0271, 0.0278, 0.0292, 0.0296, 0.0246, 0.0970, 2.8338,
    6.1716, 2.8338, 0.0970, 0.0246, 0.0296, 0.0292, 0.0278, 0.0271, 0.0255, 0.0261, 0.0267, 0.0280,
    0.0296, 0.0288, 0.0918, 1.4521, 2.9923, 1.4521, 0.0918, 0.0288, 0.0296, 0.0280, 0.0267, 0.0261,
];
