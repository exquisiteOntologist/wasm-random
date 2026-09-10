use wasm_random::random;

fn main() {
    let random_u32 = getrandom::u32().unwrap();
    println!("random u32: {}", random_u32);

    let random_u64 = getrandom::u64().unwrap();
    println!("random u64: {}", random_u64);

    let random_f32 = random::number::<f32>().unwrap();
    println!("random f32: {}", random_f32);

    let random_check = random::number::<f32>().unwrap();
    println!(
        "random f32 check: {} & {}",
        random_check,
        random_check * 100000000.
    );

    let random_f64 = random::number::<f64>().unwrap();
    println!("random f64: {}", random_f64);

    let random_i16 = random::number::<i16>().unwrap();
    println!("random i16: {}", random_i16);

    let random_i32 = random::number::<i32>().unwrap();
    println!("random i32: {}", random_i32);

    let random_i64 = random::number::<i64>().unwrap();
    println!("random i64: {}", random_i64);

    let random_u16 = random::number::<u16>().unwrap();
    println!("random u16: {}", random_u16);

    let random_u32 = random::number::<u32>().unwrap();
    println!("random u32: {}", random_u32);

    let random_u64 = random::number::<u64>().unwrap();
    println!("random u64: {}", random_u64);

    let random_range_a = random::range((100.)..1720.).unwrap();
    println!("random range a: {}", random_range_a);

    let random_range_b = random::range((0.)..0.7).unwrap();
    println!("random range b: {}", random_range_b);

    let random_range_c = random::range((0.0005)..0.0009).unwrap();
    println!("random range c: {}", random_range_c);

    let random_range_d = random::range((8000.)..24000.).unwrap();
    println!("random range d: {}", random_range_d);

    let random_range_e = random::range_all(8000..24000).unwrap();
    println!("random range e: {}", random_range_e);

    let random_range_f = random::range_all(8000..24000).unwrap();
    println!("random range f: {}", random_range_f);
}
