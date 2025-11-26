/// Generate a random number between 0 and 1.
pub fn random() -> f32 {
    let random_u32 = getrandom::u32().unwrap();
    let result = (random_u32 as f32 % 10.) / 10.;

    result
}

#[test]
fn test_random() {
    let random_number = random();
    println!("Random number: {}", random_number);
    assert!(random_number >= 0., "Random number is or is above 0.");
    assert!(random_number <= 1., "Random number is or is below 1.");
    assert!(random_number.is_finite(), "Random number is finite");
}

const PRECISION: f32 = 100000000.;

/// Generate a random number between 0 and 1 with more precision than `random()`.
/// The result is a float with more decimal places.
pub fn random_with_precision() -> f32 {
    let random_u32 = getrandom::u32().unwrap();
    let result = (random_u32 as f32 % PRECISION) / PRECISION;

    result
}

#[test]
fn test_random_with_precision() {
    let random_number = random_with_precision();
    println!("Random number: {}", random_number);
    assert!(random_number >= 0., "Random number is or is above 0.");
    assert!(random_number <= 1., "Random number is or is below 1.");
    assert!(
        random_number * 1000. >= 0.,
        "Random number is at target precision"
    );
    assert!(random_number.is_finite(), "Random number is finite");
}

/// Generate a random number between min and max.
pub fn random_from_range(min: f32, max: f32) -> f32 {
    let random_seed = random_with_precision();
    let result = min + random_seed * (max - min);

    result
}

#[test]
fn test_random_from_range_decimal() {
    let random_number = random_from_range(0., 1.);
    assert!(random_number >= 0., "Random number is or is above 0.");
    assert!(random_number <= 1., "Random number is or is below 1.");
    assert!(
        (random_number * 10. >= 0.) && (random_number * 10. <= 10.),
        "Random number is at target precision"
    );
    assert!(random_number.is_finite(), "Random number is finite");
}

#[test]
fn test_random_from_range_decimal_precise() {
    let min = 0.15;
    let max = 0.768;
    let random_number = random_from_range(min, max);
    assert!(random_number >= min, "Random number is or is above min");
    assert!(random_number <= max, "Random number is or is below max");
    assert!(random_number.is_finite(), "Random number is finite");
}

#[test]
fn test_random_from_range_integer() {
    let min = 300.;
    let max = 1500.;
    let random_number = random_from_range(min, max);
    println!("Random number: {}", random_number);
    assert!(random_number >= min, "Random number is or is above min");
    assert!(random_number <= max, "Random number is or is below max");
    assert!(random_number.is_finite(), "Random number is finite");
}
