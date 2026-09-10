use std::ops::{Add, Mul, Range, Sub};

use getrandom::Error;
use num::cast::AsPrimitive;

use crate::traits::{Float, Random};

/// Generate a random number of input type.
/// Floats return a random number between `0.` and `1.` (a fraction),
/// other number types create random numbers between `0`
/// and their maximum possible values.
///
/// # Examples
///
/// ```
/// fn main() -> Result<(), getrandom::Error> {
///     wasm_random::number::<f64>()?;
///     Ok(())
/// }
/// ```
pub fn number<T>() -> Result<T, Error>
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
/// fn main() -> Result<(), getrandom::Error> {
///     wasm_random::range((5.)..30.)?;
///     Ok(())
/// }
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

/// Generate a random number within the given range, for all
/// types of numbers.
///
/// **NOTE:** Internally performs the math operation using f64 numbers,
/// so beware of data loss when using larger numbers like f128.
///
/// **NOTE:** If your numbers are floats, use `range` instead,
/// which doesn't internally cast number types.
///
/// # Examples
///
/// ```
/// fn main() -> Result<(), getrandom::Error> {
///     wasm_random::range_all(5..30)?;
///     Ok(())
/// }
/// ```
pub fn range_all<T>(range: Range<T>) -> Result<T, Error>
where
    T: AsPrimitive<f64>,
    f64: AsPrimitive<T>,
{
    let start: f64 = range.start.as_();
    let end: f64 = range.end.as_();

    debug_assert!(start <= end);

    let random_seed = f64::random()?;
    let result = start + random_seed * (end - start);

    Ok(result.as_())
}
