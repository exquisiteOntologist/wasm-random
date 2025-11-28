use wasm_random::random;

fn main() {
    let random_u32 = getrandom::u32().unwrap();
    println!("random u32: {}", random_u32);

    let random_u64 = getrandom::u64().unwrap();
    println!("random u64: {}", random_u64);

    let random_f32 = random::random();
    println!("random f32: {}", random_f32);

    let random_f64 = random::random_f64();
    println!("random f64: {}", random_f64);

    let random_range = random::random_from_range(100., 1720.);
    println!("random range a: {}", random_range);

    let random_range_b = random::random_from_range(0., 0.7);
    println!("random range b: {}", random_range_b);
}
