use rand::Rng;

/// Public key for RSA encryption
static mut PUBLIC_KEY: i32 = 0;

/// Private key for RSA encryption  
static mut PRIVATE_KEY: i32 = 0;

/// Modulus for RSA encryption
static mut N: i32 = 0;

/// Array to store prime numbers
static mut PRIME_ARRAY: [i16; 100] = [0; 100];

/// Index for prime number array
static mut PRIME_INDEX: i8 = 0;

fn main() {
    generate_primes();

    generate_keys();

    const original_message: &str = "My name is Shashank Daima";

    println!("Original message: {}", original_message);

    let encoded_message = encrypt_message(original_message);

    println!("Encrypted message: {:?}", encoded_message);

    let decoded_message = decrypt_message(&encoded_message);

    println!("Decrypted message: {}", decoded_message);
}

/// Encrypts a message by encrypting each character
fn encrypt_message(message: &str) -> Vec<i32> {
    let mut encoded_message = Vec::new();

    for character in message.chars() {
        let encrypted = rsa_encrypt(character as i32);
        encoded_message.push(encrypted);
    }

    encoded_message
}

/// RSA encryption of a character
fn rsa_encrypt(character: i32) -> i32 {
    let mut encrypted = 1;

    unsafe {
        let mut e = PUBLIC_KEY;

        while e > 0 {
            encrypted *= character;
            encrypted %= N;
            e -= 1;
        }
    }

    encrypted
}

/// Decrypts an encoded message by decrypting each character
fn decrypt_message(encoded_message: &Vec<i32>) -> String {
    let mut decoded_message = String::new();

    for character in encoded_message {
        let decrypted = rsa_decrypt(*character);
        decoded_message.push(decrypted);
    }

    decoded_message
}

/// RSA decryption of a character
fn rsa_decrypt(character: i32) -> char {
    let mut decrypted = 1;
    let mut base = character;

    unsafe {
        let mut d = PRIVATE_KEY;

        while d > 0 {
            if d % 2 == 1 {
                decrypted = (decrypted * base) % N;
            }

            base = (base * base) % N;
            d /= 2;
        }
    }

    (decrypted as u8) as char
}

/// Generates public and private keys
fn generate_keys() {
    let mut rng = rand::thread_rng();

    let mut prime1_index: i32 = rng.gen_range(10..50);
    let prime2_index: i32 = rng.gen_range(10..50);

    while prime1_index == prime2_index {
        prime1_index = rng.gen_range(10..50);
    }

    unsafe {
        let prime1 = PRIME_ARRAY[prime1_index as usize] as i32;
        let prime2 = PRIME_ARRAY[prime2_index as usize] as i32;

        N = prime1 * prime2;

        let phi = (prime1 - 1) * (prime2 - 1);

        let mut e = 2;

        while gcd(e, phi) != 1 {
            e += 1;
        }

        PUBLIC_KEY = e;

        let mut d = 2;

        while (d * e) % phi != 1 {
            d += 1;
        }

        PRIVATE_KEY = d;
    }
}

/// Calculates greatest common divisor
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Generates an array of prime numbers
fn generate_primes() {
    let mut sieve = [true; 250];

    sieve[0] = false;
    sieve[1] = false;

    for i in 2..250 {
        for j in 2..200 {
            if i * j >= 250 {
                break;
            }
            sieve[i * j] = false;
        }
    }

    for i in 2..250 {
        if sieve[i] {
            unsafe {
                PRIME_ARRAY[PRIME_INDEX as usize] = i as i16;
                PRIME_INDEX += 1;
            }
        }
    }
}
