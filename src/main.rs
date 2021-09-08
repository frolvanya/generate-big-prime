use rand::{prelude::SliceRandom, Rng};

fn mod_pow(
    mut base: primitive_types::U512,
    mut exp: primitive_types::U512,
    modulus: primitive_types::U512,
) -> primitive_types::U512 {
    if modulus == primitive_types::U512::from(1) {
        return primitive_types::U512::from(0);
    }
    let mut result = primitive_types::U512::from(1);
    base = base % modulus;
    while exp > primitive_types::U512::from(0) {
        if exp % 2 == primitive_types::U512::from(1) {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base.pow(primitive_types::U512::from(2)) % modulus
    }
    result
}

fn low_level_prime(n: primitive_types::U512) -> bool {
    for divisor in vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349,
    ] {
        if n % divisor == primitive_types::U512::from(0)
            && primitive_types::U512::from(divisor).pow(primitive_types::U512::from(2)) <= n
        {
            return false;
        }
    }

    true
}

fn high_level_prime(n: primitive_types::U512, k: i64) -> bool {
    let allowed_bits = vec![8, 32, 64, 128];

    if n == primitive_types::U512::from(2) || n == primitive_types::U512::from(3) {
        return true;
    }

    if n <= primitive_types::U512::from(1) || n % 2 == primitive_types::U512::from(0) {
        return false;
    }

    let mut s = 0;
    let mut r = n - 1;

    while r & primitive_types::U512::from(1) == primitive_types::U512::from(0) {
        s += 1;
        r /= 2;
    }

    for _ in 0..k {
        let random_bit = allowed_bits.choose(&mut rand::thread_rng()).unwrap();
        let random_number = generate_n_bit_number(*random_bit);

        // let mut x = random_number.pow(r) % n;
        let mut x = mod_pow(random_number, r, n);

        if x != primitive_types::U512::from(1) && x != n - 1 {
            let mut j = 1;
            while j < s && x != n - 1 {
                // x = x.pow(primitive_types::U512::from(2)) % n;
                x = mod_pow(x, primitive_types::U512::from(2), n);

                if x == primitive_types::U512::from(1) {
                    return false;
                }
                j += 1;
            }
            if x != n - 1 {
                return false;
            }
        }
        return true;
    }

    true
}

fn generate_n_bit_number(bits: u32) -> primitive_types::U512 {
    let mut rng = rand::thread_rng();
    let mut bytes = Vec::new();

    for _ in 0..bits / 8 {
        bytes.push(rng.gen::<u8>());
    }

    primitive_types::U512::from_little_endian(&bytes)
}

fn main() {
    loop {
        let prime_candidate = generate_n_bit_number(256);

        if low_level_prime(prime_candidate) && high_level_prime(prime_candidate, 128) {
            println!("{}", prime_candidate);
            break;
        }
    }
}
