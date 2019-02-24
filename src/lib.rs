
use std::fs::File;
use std::path::Path;
use std::io::{Error, Read};

use num_complex::Complex;

/// Read entire file into f32 vector. Will drop any excess bytes.
pub fn from_file_raw_f32(p: &Path) -> Result<Vec<f32>, Error> {
    let mut file = File::open(p)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let res = buffer
        .chunks_exact(4) // TODO use size_of
        .map(|sl| 
             f32::from_bits(
                 u32::from_ne_bytes([sl[0], sl[1], sl[2], sl[3]])
             )
         ) // TODO handle endianness
        .collect();

    Ok(res)
}

/// Read entire file into Complex<f32> values.
pub fn from_file_complex_f32(p: &Path) -> Result<Vec<Complex<f32>>, Error> {
    let mut file = File::open(p)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let res = buffer
        .chunks_exact(8) // TODO use size_of
        .map(|sl| {
             let re = f32::from_bits(
                 u32::from_ne_bytes([sl[0], sl[1], sl[2], sl[3]])
             );
             let im = f32::from_bits(
                 u32::from_ne_bytes([sl[4], sl[5], sl[6], sl[7]])
             );
             Complex::new(re, im)
        }
        )
        .collect();

    Ok(res)
}

pub fn f32_to_complex_vec(mut floats: Vec<f32>) -> Vec<Complex<f32>> {
    // The following checks ensure the preconditions of the unsafe block.
    // Most of this code will go away at compile time.

    assert_eq!(std::mem::align_of::<f32>(), std::mem::align_of::<Complex<f32>>());

    let size_diff = std::mem::size_of::<Complex<f32>>() / std::mem::size_of::<f32>();
    let size_rem = std::mem::size_of::<Complex<f32>>() % std::mem::size_of::<f32>();
    assert_eq!(size_rem, 0);
    assert_eq!(floats.len() % size_diff, 0);
    assert_eq!(floats.capacity() % size_diff, 0);

    unsafe {
        let len = floats.len() / size_diff;
        let capacity = floats.capacity() / size_diff;
        let ptr = floats.as_mut_ptr() as *mut Complex<f32>;

        std::mem::forget(floats);

        Vec::from_raw_parts(ptr, len, capacity)
    }
}

/// WARNING!! Do not use this function. It is not guaranteed that the following is correct.
///
/// This function will convert a vector's values from T to U via pointer manipulation and casting.
///
/// This function is only safe to call if the following conditions are true:
///   1. T and U have the same alignment
///   2. v has the exact number of bytes required for U to fill
///     - len and capacity must be correct
///   3. All valid bit-patterns of T must be valid bit-patterns of U
///
/// If 1 or 2 are violated this function will panic at run time.
/// This function cannot check for 3.
pub fn vec_convert<T, U>(mut v: Vec<T>) -> Vec<U> {
    // TODO determine if this can be relaxed if U has a smaller alignment than T
    assert_eq!(std::mem::align_of::<T>(), std::mem::align_of::<U>());

    let t_size = std::mem::size_of::<T>();
    let u_size = std::mem::size_of::<U>();

    // TODO this requires t_size < u_size, is this necessary?
    // TODO is it necessary that individual elements overlap exactly?
    // TODO should I check t_size <= u_size?
    assert_eq!(u_size % t_size, 0);

    // There cannot be any leftover bytes
    assert_eq!(v.len() * t_size % u_size, 0);
    assert_eq!(v.capacity() * t_size % u_size, 0);

    let len = v.len() * t_size / u_size;
    let cap = v.capacity() * t_size / u_size;
    let ptr = v.as_mut_ptr();
    
    unsafe {
        let ptr = ptr as *mut U;

        std::mem::forget(v);

        Vec::from_raw_parts(ptr, len, cap)
    }
}

#[cfg(test)]
mod test {
    use num_complex::Complex;
    use super::*;

    #[test]
    /// Test simple vector conversion between f32 and Complex<f32> vectors.
    fn basic_vec_conversion_test() {
        let given = vec![0.0, 1.0, 3.4, 1.1, 4.3, 3.2];
        let expected: Vec<Complex<f32>> = given.chunks(2).map(|sl| Complex::new(sl[0], sl[1])).collect();

        assert_eq!(f32_to_complex_vec(given), expected);
    }
}
