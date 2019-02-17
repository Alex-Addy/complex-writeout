#[macro_use]
extern crate criterion;

use criterion::{Criterion, Bencher, Fun};

use num_complex::Complex;

// Convert from float array to complex

fn convert_via_chunks(floats: &[f32]) -> Vec<Complex<f32>> {
    floats
        .chunks(2)
        .map(|chunk| if chunk.len() == 2 {
            Complex::new(chunk[0], chunk[1])
        } else { 
            Complex::new(chunk[0], 0.0)
        }
        ).collect()
}

fn convert_via_chunks_exact(floats: &[f32]) -> Vec<Complex<f32>> {
    floats
        .chunks_exact(2)
        .map(|chunk| Complex::new(chunk[0], chunk[1]))
        .collect()
}

fn convert_via_reconstruction(floats: &[f32]) -> Vec<Complex<f32>> {
    // This code is *probably* not invoking undefined behavior. If you really need this performance
    // get some smart people to review this.

    // This will allocate a new Vec with len == capacity == floats.len()
    let v = floats.to_vec();
    let len = v.len();

    // The following unsafe code requires this to be true, if there is an extra value then
    // undefined behavior *will* occur.
    assert_eq!(len % 2, 0);

    unsafe {
        // this will cause shrink_to_fit which will have nothing to do
        let mut s = v.into_boxed_slice();
        let ptr = s.as_mut_ptr();
        // Without this *ptr will be freed at the end of this block, resulting in a Vec pointing to
        // freed memory
        std::mem::forget(s);

        let ptr = ptr as *mut Complex<f32>;

        Vec::from_raw_parts(ptr, len/2, len/2)
    }
}

// Benchmarking functions

fn bench_convert_via_chunks(b: &mut Bencher, i: &Vec<f32>) {
    b.iter(|| {
        convert_via_chunks(i);
    });
}

fn bench_convert_via_chunks_exact(b: &mut Bencher, i: &Vec<f32>) {
    b.iter(|| {
        convert_via_chunks_exact(i);
    });
}

fn bench_convert_via_reconstruction(b: &mut Bencher, i: &Vec<f32>) {
    b.iter(|| {
        convert_via_reconstruction(i);
    });
}

fn compare_conversions(c: &mut Criterion) {
    let chunk_conv = Fun::new("Chunks", bench_convert_via_chunks);
    let exact_conv = Fun::new("Chunks Exact", bench_convert_via_chunks_exact);
    let raw_conv = Fun::new("Raw Parts", bench_convert_via_reconstruction);
    let funs = vec![chunk_conv, exact_conv, raw_conv];

    let input = vec![0.0f32; 2048];

    c.bench_functions("Convert Slice to new Complexes", funs, input);
}

criterion_group!(benches, compare_conversions);
criterion_main!(benches);

