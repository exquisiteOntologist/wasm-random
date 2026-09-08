use std::ops::{Add, Mul, Range, Sub};

use getrandom::Error;

use crate::traits::{Float, Random};

/// Generate a random number `f32` between `0.` and `1.`
pub fn f32() -> Result<f32, Error> {
    f32::random()
}

/// Generate a random number `f64` between `0.` and `1.`
pub fn f64() -> Result<f64, Error> {
    f64::random()
}

pub fn i16() -> Result<i16, Error> {
    i16::random()
}

pub fn i32() -> Result<i32, Error> {
    i32::random()
}

pub fn i64() -> Result<i64, Error> {
    i64::random()
}

pub fn u16() -> Result<u16, Error> {
    u16::random()
}

pub fn u32() -> Result<u32, Error> {
    u32::random()
}

pub fn u64() -> Result<u64, Error> {
    u64::random()
}

/// Generate a random number of input type between `0` and `1`
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// wasm_random::random::<f64>()?;
/// # Ok(()) }
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
/// # fn main() -> Result<(), getrandom::Error> {
/// wasm_random::range((5.)..30.)?;
/// # Ok(()) }
/// ```
pub fn range<T>(range: Range<T>) -> Result<T, Error>
where
    T: Float + Random + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    debug_assert!(range.start <= range.end);

    let random_seed = T::random()?;
    let result = range.start + random_seed * (range.end - range.start);

    Ok(result)
}

// /// Generate a random number within the given range, for all
// /// types of numbers.
// /// Internally performs the math operation using f64 numbers.
// /// Unfortunately none of the primitives implement Into for conversion.
// ///
// /// **NOTE:** You probably want `range` instead of `range_all`.
// ///
// /// # Examples
// ///
// /// ```
// /// # fn main() -> Result<(), getrandom::Error> {
// /// wasm_random::range_all(5..30)?;
// /// # Ok(()) }
// /// ```
// pub fn range_all<T>(range: Range<T>) -> Result<T, Error>
// where
//     // Unfortunately i64 etc don't implement From<f64> or Into<f64>.
//     // The `as` keyword can perform the cast while `from` won't work, however,
//     // `as` castability can't be specified here.
//     T: Into<f64> + From<f64>,
//     f64: Into<T> + From<T>,
// {
//     let start: f64 = range.start.into();
//     let end: f64 = range.end.into();

//     debug_assert!(start <= end);

//     let random_seed = f64::random()?;
//     let result = start + random_seed * (end - start);

//     Ok(result.into())
// }

pub fn range_i64(range: Range<i64>) -> Result<i64, Error> {
    let start = range.start as f64;
    let end = range.end as f64;

    debug_assert!(start <= end);

    let random_seed = f64::random()?;
    let result = start + random_seed * (end - start);

    Ok(result as i64)
}

pub fn range_u64(range: Range<u64>) -> Result<u64, Error> {
    let start = range.start as f64;
    let end = range.end as f64;

    debug_assert!(start >= 0.);
    debug_assert!(start <= end);

    let random_seed = f64::random()?;
    let result = start + random_seed * (end - start);

    Ok(result as u64)
}
