use getrandom::Error;

use crate::constants::{PRECISION_F32, PRECISION_F64};

/// Can generate random numbers
pub trait Random: Sized {
    /// Generate a random number
    fn random() -> Result<Self, Error>;
}

impl Random for f32 {
    /// Generate a random number between 0. and 1.
    fn random() -> Result<Self, Error> {
        let random_u32 = getrandom::u32()?;
        let result = (random_u32 as f32 % PRECISION_F32) / PRECISION_F32;

        Ok(result)
    }
}

impl Random for f64 {
    /// Generate a random number between 0. and 1.
    fn random() -> Result<Self, Error> {
        let random_u64 = getrandom::u64()?;
        let result = (random_u64 as f64 % PRECISION_F64) / PRECISION_F64;

        Ok(result)
    }
}

impl Random for i16 {
    /// Generate a random number.
    /// **Note:** Not between 0 and 1.
    fn random() -> Result<Self, Error> {
        Ok((getrandom::u32()? as i16).abs())
    }
}

impl Random for i32 {
    /// Generate a random number.
    /// **Note:** Not between 0 and 1.
    fn random() -> Result<Self, Error> {
        Ok((getrandom::u32()? as i32).abs())
    }
}

impl Random for i64 {
    /// Generate a random number.
    /// **Note:** Not between 0 and 1.
    fn random() -> Result<Self, Error> {
        Ok((getrandom::u64()? as i64).abs())
    }
}

// impl Random for i128 {
//     /// Generate a random number.
//     /// **Note:** Not between 0 and 1.
//     fn random() -> Result<Self, Error> {
//         Ok(getrandom::u128()? as i128)
//     }
// }

impl Random for u16 {
    /// Generate a random number.
    /// **Note:** Not between 0 and 1.
    fn random() -> Result<Self, Error> {
        Ok(getrandom::u32()? as u16)
    }
}

impl Random for u32 {
    /// Generate a random number.
    /// **Note:** Not between 0 and 1.
    fn random() -> Result<Self, Error> {
        getrandom::u32()
    }
}

impl Random for u64 {
    /// Generate a random number.
    /// **Note:** Not between 0 and 1.
    fn random() -> Result<Self, Error> {
        getrandom::u64()
    }
}

// impl Random for u128 {
//     /// Generate a random number.
//     /// Note: Not between 0 and 1.
//     fn random() -> Result<Self, Error> {
//         Ok(getrandom::u128()? as u128)
//     }
// }

/// Floating-point numbers
pub trait Float {}

impl Float for f32 {}
impl Float for f64 {}

// Can't implement for primitives
// impl Into<f64> for f32 {
//     fn into(v: f32) {
//         v as f64
//     }
// }
