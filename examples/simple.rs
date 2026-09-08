use wasm_random::{f32, f64, range};

fn main() {
    let random_u32 = getrandom::u32().unwrap();
    println!("random u32: {}", random_u32);

    let random_u64 = getrandom::u64().unwrap();
    println!("random u64: {}", random_u64);

    let random_f32 = f32().unwrap();
    println!("random f32: {}", random_f32);

    let random_check = wasm_random::f32().unwrap();
    println!(
        "random f32 check: {} & {}",
        random_check,
        random_check * 100000000.
    );

    let random_f64 = f64().unwrap();
    println!("random f64: {}", random_f64);

    let random_range_a = range((100.)..1720.).unwrap();
    println!("random range a: {}", random_range_a);

    let random_range_b = wasm_random::range((0.)..0.7).unwrap();
    println!("random range b: {}", random_range_b);

    let random_range_c = range((0.0005)..0.0009).unwrap();
    println!("random range c: {}", random_range_c);

    let random_range_d = wasm_random::range((8000.)..24000.).unwrap();
    println!("random range d: {}", random_range_d);

    let random_range_e = wasm_random::range_i64(8000..24000).unwrap();
    println!("random range e: {}", random_range_e);
}
