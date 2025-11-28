const PRECISION_F32: f32 = 100000000.;

/// Generate a random number `f32` between `0.` and `1.`
pub fn random() -> f32 {
    let random_u32 = getrandom::u32().unwrap();
    let result = (random_u32 as f32 % PRECISION_F32) / PRECISION_F32;

    result
}

#[test]
fn test_random() {
    let random_number = random();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * PRECISION_F32) >= 1.);
    assert!(random_number.is_finite());
}

const PRECISION_F64: f64 = 10000000000000000000.;

/// Generate a random number `f64` between `0.` and `1.`
pub fn random_f64() -> f64 {
    let random_u64 = getrandom::u64().unwrap();
    let result = (random_u64 as f64 % PRECISION_F64) / PRECISION_F64;

    result
}

#[test]
fn test_random_f64() {
    let random_number = random_f64();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * PRECISION_F64) >= 1.);
    assert!(random_number.is_finite());
}

/// Generate a random number between min and max.
pub fn random_from_range(min: f32, max: f32) -> f32 {
    debug_assert!(min <= max);

    let random_seed = random();
    let result = min + random_seed * (max - min);

    result
}

#[test]
fn test_random_from_range_a() {
    let random_number = random_from_range(0., 1.);
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
    assert!(random_number.is_finite());
}

#[test]
fn test_random_from_range_b() {
    let min = 0.15;
    let max = 0.768;
    let random_number = random_from_range(min, max);
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_random_from_range_c() {
    let min = 0.35;
    let max = 0.42;
    let random_number = random_from_range(min, max);
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_random_from_range_d() {
    let min = 300.;
    let max = 1500.;
    let random_number = random_from_range(min, max);
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

/// Generate a random number between min and max.
pub fn random_from_range_f64(min: f64, max: f64) -> f64 {
    debug_assert!(min <= max);

    let random_seed = random_f64();
    let result = min + random_seed * (max - min);

    result
}

#[test]
fn test_random_from_range_f64_a() {
    let random_number = random_from_range_f64(0., 1.);
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
    assert!(random_number.is_finite());
}
