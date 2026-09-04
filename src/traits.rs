use getrandom::Error;

use crate::constants::{PRECISION_F32, PRECISION_F64};

/// Can generate random numbers
pub trait Random: Sized {
    fn random() -> Result<Self, Error>;
}

impl Random for f32 {
    fn random() -> Result<Self, Error> {
        let random_u32 = getrandom::u32()?;
        let result = (random_u32 as f32 % PRECISION_F32) / PRECISION_F32;

        Ok(result)
    }
}

impl Random for f64 {
    fn random() -> Result<Self, Error> {
        let random_u64 = getrandom::u64()?;
        let result = (random_u64 as f64 % PRECISION_F64) / PRECISION_F64;

        Ok(result)
    }
}
