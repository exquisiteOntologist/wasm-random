use std::ops::{Add, Mul, Range, Sub};

const PRECISION_F32: f32 = 100000000.;

/// Generate a random number `f32` between `0.` and `1.`
pub fn random() -> f32 {
    let random_u32 = getrandom::u32().unwrap();
    let result = (random_u32 as f32 % PRECISION_F32) / PRECISION_F32;

    result
}

const PRECISION_F64: f64 = 10000000000000000000.;

/// Generate a random number `f64` between `0.` and `1.`
pub fn random_f64() -> f64 {
    let random_u64 = getrandom::u64().unwrap();
    let result = (random_u64 as f64 % PRECISION_F64) / PRECISION_F64;

    result
}

/// Generate a random number between min and max.
pub fn random_from_range(min: f32, max: f32) -> f32 {
    debug_assert!(min <= max);

    let random_seed = random();
    let result = min + random_seed * (max - min);

    result
}

/// Generate a random number between min and max.
pub fn random_from_range_f64(min: f64, max: f64) -> f64 {
    debug_assert!(min <= max);

    let random_seed = random_f64();
    let result = min + random_seed * (max - min);

    result
}

/// Generate a random number within the given range.
pub fn random_from_range_exp(range: Range<i64>) -> i64 {
    let start = range.start as f64;
    let end = range.end as f64;

    debug_assert!(start <= end);

    let random_seed = random_f64();
    let result = start + random_seed * (end - start);

    result as i64
}

/// Generate a random number within the given range.
pub fn random_from_range_exp_f64(range: Range<f64>) -> f64 {
    debug_assert!(range.start <= range.end);

    let random_seed = random_f64();
    let result = range.start + random_seed * (range.end - range.start);

    result
}

// pub fn random_from_range_exp<T>(range: Range<T>) -> T
// where
//     // can convert T Into f32
//     // can conver From f32 to T
//     T: Into<f32> + From<f32> + Copy,
// {
//     let start: f32 = range.start.into();
//     let end: f32 = range.end.into();

//     debug_assert!(start <= end);

//     let random_seed = random();
//     let result = start + random_seed * (end - start);

//     result.into()
// }

#[cfg(test)]
mod tests {
    use crate::{
        random::{
            PRECISION_F32, PRECISION_F64, random, random_f64, random_from_range,
            random_from_range_f64,
        },
        random_from_range_exp, random_from_range_exp_f64,
    };

    #[test]
    fn test_random() {
        let random_number = random();
        assert!(random_number >= 0.);
        assert!(random_number <= 1.);
        assert!((random_number * PRECISION_F32) >= 1.);
        assert!(random_number.is_finite());
    }

    #[test]
    fn test_random_f64() {
        let random_number = random_f64();
        assert!(random_number >= 0.);
        assert!(random_number <= 1.);
        assert!((random_number * PRECISION_F64) >= 1.);
        assert!(random_number.is_finite());
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

    #[test]
    fn test_random_from_range_f64_a() {
        let random_number = random_from_range_f64(0., 1.);
        assert!(random_number >= 0.);
        assert!(random_number <= 1.);
        assert!((random_number * 10. >= 0.) && (random_number * 10. <= 10.),);
        assert!(random_number.is_finite());
    }

    #[test]
    fn test_random_from_range_exp_a() {
        let random_number = random_from_range_exp(0..10);
        // println!("exp_a {}", random_number);
        assert!(random_number >= 0);
        assert!(random_number <= 10);
    }

    #[test]
    fn test_random_from_range_exp_f64_a() {
        let random_number = random_from_range_exp_f64((0.)..10.);
        // println!("exp_f64_a {}", random_number);
        assert!(random_number >= 0.);
        assert!(random_number <= 10.);
    }
}
