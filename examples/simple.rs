use wasm_random::{random, random_f64, random_from_range, random_from_range_f64};

fn main() {
    let random_u32 = getrandom::u32().unwrap();
    println!("random u32: {}", random_u32);

    let random_u64 = getrandom::u64().unwrap();
    println!("random u64: {}", random_u64);

    let random_f32 = random();
    println!("random f32: {}", random_f32);

    let random_check = wasm_random::random();
    println!(
        "random f32 check: {} & {}",
        random_check,
        random_check * 100000000.
    );

    let random_f64 = random_f64();
    println!("random f64: {}", random_f64);

    let random_range = random_from_range(100., 1720.);
    println!("random range a: {}", random_range);

    let random_range_b = wasm_random::random_from_range(0., 0.7);
    println!("random range b: {}", random_range_b);

    let random_range_c = random_from_range_f64(0.0005, 0.0009);
    println!("random range c: {}", random_range_c);

    let random_range_d = wasm_random::random_from_range_f64(8000., 24000.);
    println!("random range d: {}", random_range_d);
}
