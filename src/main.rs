mod padding;
mod primes;
mod rsa;
mod sha2;

use crate::rsa::{decrypt, encrypt, generate_keypair};
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{One, ToBytes, Zero};
use rand::{thread_rng, Rng};
use std::ops::{Mul, Rem};

pub fn print_digest(digest: &[u8]) {
    for byte in digest {
        print!("{:02x}", byte);
    }
    println!(" ");
}

fn main() {
    let (n, e, d) = generate_keypair(1024);

    let ciphertext = encrypt(b"Hello, world!", b"", &n, &e);

    print!("Zaszyfrowana wiadomość: ");
    print_digest(ciphertext.to_be_bytes().as_slice());

    let (decrypted_message, _) = decrypt(&ciphertext, &n, &d);
    println!(
        "Odszyfrowana wiadomość: {}",
        String::from_utf8_lossy(decrypted_message.as_slice())
    );
}
