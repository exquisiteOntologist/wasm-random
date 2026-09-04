use std::ops::{Add, Mul, Range, Sub};

use getrandom::Error;

pub const PRECISION_F32: f32 = 100000000.;

/// Generate a random number `f32` between `0.` and `1.`
pub fn f32() -> Result<f32, Error> {
    let random_u32 = getrandom::u32()?;
    let result = (random_u32 as f32 % PRECISION_F32) / PRECISION_F32;

    Ok(result)
}

pub const PRECISION_F64: f64 = 10000000000000000000.;

/// Generate a random number `f64` between `0.` and `1.`
pub fn f64() -> Result<f64, Error> {
    let random_u64 = getrandom::u64()?;
    let result = (random_u64 as f64 % PRECISION_F64) / PRECISION_F64;

    Ok(result)
}

/// Generate a random number within the given range.
pub fn range_f32(range: Range<f32>) -> Result<f32, Error> {
    debug_assert!(range.start <= range.end);

    let random_seed = f32()?;
    let result = range.start + random_seed * (range.end - range.start);

    Ok(result)
}

/// Generate a random number within the given range.
pub fn range_f64(range: Range<f64>) -> Result<f64, Error> {
    debug_assert!(range.start <= range.end);

    let random_seed = f64()?;
    let result = range.start + random_seed * (range.end - range.start);

    Ok(result)
}

/// Generate a random number within the given range.
pub fn range_i64(range: Range<i64>) -> Result<i64, Error> {
    let start = range.start as f64;
    let end = range.end as f64;

    debug_assert!(start <= end);

    let random_seed = f64()?;
    let result = start + random_seed * (end - start);

    Ok(result as i64)
}
