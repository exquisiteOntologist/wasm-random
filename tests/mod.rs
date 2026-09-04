use wasm_random::random;

#[test]
fn test_f32() {
    let random_number = random::f32().unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * random::PRECISION_F32) >= 1.);
    assert!(random_number.is_finite());
}

#[test]
fn test_f64() {
    let random_number = random::f64().unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * random::PRECISION_F64) >= 1.);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_a() {
    let random_number = random::range_f32((0.)..1.).unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_b() {
    let min = 0.15;
    let max = 0.768;
    let random_number = random::range_f32(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_c() {
    let min = 0.35;
    let max = 0.42;
    let random_number = random::range_f32(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_d() {
    let min = 300.;
    let max = 1500.;
    let random_number = random::range_f32(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f64_a() {
    let random_number = random::range_f64((0.)..1.).unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f64_b() {
    let random_number = random::range_f64((0.)..10.).unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 10.);
}

#[test]
fn test_range_i64() {
    let random_number = random::range_i64(0..10).unwrap();
    assert!(random_number >= 0);
    assert!(random_number <= 10);
}
