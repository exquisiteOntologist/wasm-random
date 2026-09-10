use wasm_random::{
    constants::{PRECISION_F32, PRECISION_F64},
    random,
};

#[test]
fn test_f32() {
    let random_number = random::number::<f32>().unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * PRECISION_F32) >= 1.);
    assert!(random_number.is_finite());
}

#[test]
fn test_f64() {
    let random_number = random::number::<f64>().unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * PRECISION_F64) >= 1.);
    assert!(random_number.is_finite());
}

#[test]
fn test_i16() {
    let random_number = random::number::<i16>().unwrap();
    assert!(random_number > 0);
}

#[test]
fn test_i32() {
    let random_number = random::number::<i32>().unwrap();
    assert!(random_number > 0);
}

#[test]
fn test_i64() {
    let random_number = random::number::<i64>().unwrap();
    assert!(random_number > 0);
}

#[test]
fn test_u16() {
    let random_number = random::number::<u16>().unwrap();
    assert!(random_number > 0);
}

#[test]
fn test_u32() {
    let random_number = random::number::<u32>().unwrap();
    assert!(random_number > 0);
}

#[test]
fn test_u64() {
    let random_number = random::number::<u64>().unwrap();
    assert!(random_number > 0);
}

#[test]
fn test_generic_random() {
    let random_number = random::number::<f64>().unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * PRECISION_F64) >= 1.);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_a() {
    let random_number = random::range((0.)..1.).unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
}

#[test]
fn test_range_f32_b() {
    let min = 0.15 as f32;
    let max = 0.768 as f32;
    let random_number = random::range(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_c() {
    let min = 0.35 as f32;
    let max = 0.42 as f32;
    let random_number = random::range(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f32_d() {
    let min = 300. as f32;
    let max = 1500. as f32;
    let random_number = random::range(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f64_a() {
    let random_number = random::range((0. as f64)..1. as f64).unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 1.);
    assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
    assert!(random_number.is_finite());
}

#[test]
fn test_range_f64_b() {
    let random_number = random::range((0. as f64)..10. as f64).unwrap();
    assert!(random_number >= 0.);
    assert!(random_number <= 10.);
}

#[test]
fn test_range_i16() {
    let min = 0 as i16;
    let max = i16::MAX as i16;
    let random_number = random::range_all(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(min != max);
}

#[test]
fn test_range_i32() {
    let min = i16::MAX as i32;
    let max = i32::MAX as i32;
    let random_number = random::range_all(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(min != max);
}

#[test]
fn test_range_i64() {
    let min = i32::MAX as i64;
    let max = i64::MAX as i64;
    let random_number = random::range_all(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(min != max);
}

#[test]
fn test_range_u32() {
    let min = 0 as u32;
    let max = u32::MAX as u32;
    let random_number = random::range_all(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(min != max);
}

#[test]
fn test_range_u64() {
    let min = u32::MAX as u64;
    let max = u64::MAX as u64;
    let random_number = random::range_all(min..max).unwrap();
    assert!(random_number >= min);
    assert!(random_number <= max);
    assert!(min != max);
}
