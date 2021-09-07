use rand::Rng;

fn generate_possible_prime(bits: u32) -> primitive_types::U512 {
    let mut rng = rand::thread_rng();
    let mut bytes = Vec::new();

    for _ in 0..bits / 8 {
        bytes.push(rng.gen::<u8>());
    }
    println!("{:?}\n\n{}", bytes, bytes.len());
    primitive_types::U512::from_little_endian(&bytes)
}

fn main() {
    println!("{}", generate_possible_prime(512));
}
