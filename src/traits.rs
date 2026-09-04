use getrandom::Error;

/// Can generate random numbers
pub trait Random: Sized {
    fn random() -> Result<Self, Error>;
}

impl Random for f32 {
    fn random() -> Result<Self, Error> {
        Ok(0.)
    }
}

pub fn experiment() -> Result<f32, Error> {
    Ok(f32::random()?)
}
