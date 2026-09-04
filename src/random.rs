use std::ops::{Add, Mul, Range, Sub};

use getrandom::Error;

use crate::traits::Random;

/// Generate a random number `f32` between `0.` and `1.`
pub fn f32() -> Result<f32, Error> {
    f32::random()
}

/// Generate a random number `f64` between `0.` and `1.`
pub fn f64() -> Result<f64, Error> {
    f64::random()
}

/// Generate a random number of input type between `0` and `1`
///
/// # Examples
///
/// ```
/// random::<f64>()
/// ```
pub fn random<T>() -> Result<T, Error>
where
    T: Random,
{
    T::random()
}

/// Generate a random number within the given range.
///
/// # Examples
///
/// ```
/// range((5.)..30.)
/// ```
pub fn range<T>(range: Range<T>) -> Result<T, Error>
where
    T: Random + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    debug_assert!(range.start <= range.end);

    let random_seed = T::random()?;
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
