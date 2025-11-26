use wasm_random::random;

fn main() {
    let random_u32 = getrandom::u32().unwrap();
    println!("random u32: {}", random_u32);

    let random_not_precise = random::random();
    println!("inprecise random f32: {}", random_not_precise);

    let random_precise = random::random_more_precision();
    println!("precise random f32: {}", random_precise);

    let random_range = random::random_from_range(100., 1720.);
    println!("random range a: {}", random_range);

    let random_range_b = random::random_from_range(0., 0.7);
    println!("random range b: {}", random_range_b);
}
