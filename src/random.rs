/// Generate a random number between 0 and 1.
pub fn random() -> f32 {
    let random_u32 = getrandom::u32().unwrap();
    let result = (random_u32 as f32 % 10.) / 10.;

    result
}

/// Generate a random number between 0 and 1 with more precision.
/// The result is a float with more decimal places.
pub fn random_more_precision() -> f32 {
    let random_u32 = getrandom::u32().unwrap();
    let result = (random_u32 as f32 % 1000.) / 1000.;

    result
}

/// Generate a random number between min and max.
pub fn random_from_range(min: f32, max: f32) -> f32 {
    let random_seed = random();
    let result = min + random_seed * (max - min);

    result
}
