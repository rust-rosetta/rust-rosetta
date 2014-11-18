// Implements http://rosettacode.org/wiki/Fast_Fourier_transform

extern crate num;

use std::num::Float;
use num::complex::Complex;

fn fft(arr: &[Complex<f32>]) -> Vec<Complex<f32>> {
    if arr.len() <= 1 {
        return arr.to_vec()
    }

    let mut even = Vec::with_capacity(arr.len()/2);
    let mut odd = Vec::with_capacity(arr.len()/2);

    for i in range(0u, arr.len()) {
        if i % 2 == 0 {
            even.push(arr[i].clone());
        } else {
            odd.push(arr[i].clone());
        }
    }

    let even_fft = fft(even.as_slice());
    let odd_fft = fft(odd.as_slice());

    let mut out = Vec::from_elem(arr.len(), Complex::new(0f32, 0f32));
    for i in range(0u, arr.len()/2) {
        let twiddle: Complex<f32> = Complex::from_polar(&1f32, &(-2f32*Float::pi()*(i as f32)/(arr.len() as f32)));
        out[i] = even_fft[i] + twiddle*odd_fft[i];
        out[i + arr.len()/2] = even_fft[i] - twiddle*odd_fft[i];
    }

    out
}

#[cfg(not(test))]
fn main() {
    let test = [
        Complex::new(1f32, 0f32),
        Complex::new(1f32, 0f32),
        Complex::new(1f32, 0f32),
        Complex::new(1f32, 0f32),
        Complex::new(0f32, 0f32),
        Complex::new(0f32, 0f32),
        Complex::new(0f32, 0f32),
        Complex::new(0f32, 0f32)
    ];

    let test_fft = fft(test.as_slice());
    println!("{}", test_fft);
}

#[cfg(test)]
mod test {
    use super::fft;
    use num::complex::Complex;

    #[test]
    fn transform() {
        let test = [
            Complex::new(1f32, 0f32),
            Complex::new(1f32, 0f32),
            Complex::new(1f32, 0f32),
            Complex::new(1f32, 0f32),
            Complex::new(0f32, 0f32),
            Complex::new(0f32, 0f32),
            Complex::new(0f32, 0f32),
            Complex::new(0f32, 0f32)
        ];
        let target = [
            Complex::new(4f32, 0f32),
            Complex::new(1f32, -2.414f32),
            Complex::new(0f32, 0f32),
            Complex::new(1f32, -0.414f32),
            Complex::new(0f32, 0f32),
            Complex::new(1f32, 0.414f32),
            Complex::new(0f32, 0f32),
            Complex::new(1f32, 2.414f32)
        ];

        let test_fft = fft(test.as_slice());
        println!("{}", target.to_vec());
        println!("{}", test_fft);
        for (test_item, target_item) in test_fft.iter().zip(target.iter()) {
            assert!((*test_item - *target_item).norm_sqr() < 1e-6);
        }
    }
}
