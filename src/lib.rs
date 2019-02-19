
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

