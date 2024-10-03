use crate::primes::rand_prime;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, ToBytes, Zero};
use crate::padding;

pub fn nwd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while b != BigUint::zero() {
        let t =  &a % &b;
        a = b;
        b = t;
    }

    a
}

/// Generuje klucz RSA.
pub fn generate_keypair(bits: usize) -> (BigUint, BigUint, BigUint) {
    let mut rng = rand::thread_rng();

    // Krok 1: Generujemy dwie losowe liczby pierwsze p i q.
    let p = rand_prime(&mut rng, bits / 2);
    let q = rand_prime(&mut rng, bits / 2);
    let n = &p * &q;

    // Krok 2: Obliczamy λ(n) = NWW(λ(p), λ(q)) = NWW((p-1), (q-1)) = (p-1)(q-1) / NWD(p-1, q-1)
    let p_1 = &p - 1u8;
    let q_1 = &q - 1u8;
    let lambda = (&p_1 * &q_1) / nwd(p_1, q_1);

    // Krok 3: Wybieramy publiczny wykładnik e względnie pierwszy do λ(n) (najczęściej 65537)
    let e = 65537.to_biguint().unwrap();

    // Krok 4: Wybieramy prywatny wykładnik d, gdzie d ≡ e^(-1) mod λ(n)
    let d = e.modinv(&lambda).unwrap();

    (n, e, d)
}

pub fn encrypt(message: &[u8], label: &[u8], n: &BigUint, e: &BigUint) -> BigUint {
    let padded = padding::encode(message, label);

    encrypt_without_padding(padded.as_slice(), n, e)
}
pub fn decrypt(ciphertext: &BigUint, n: &BigUint, d: &BigUint) -> (Vec<u8>, Vec<u8>) {
    let decrypted = decrypt_without_padding(ciphertext, n, d);

    padding::decode(decrypted.as_slice())
}

pub fn encrypt_without_padding(message: &[u8], n: &BigUint, e: &BigUint) -> BigUint {
    BigUint::from_bytes_be(message).modpow(e, n)
}

pub fn decrypt_without_padding(ciphertext: &BigUint, n: &BigUint, d: &BigUint) -> Vec<u8> {
    let int = ciphertext.modpow(d, n);
    let size = (((n.bits() - 1) / 8) + 1) as usize;

    let mut bytes = int.to_be_bytes();

    let mut result = Vec::with_capacity(size);

    for _ in 0..(size - bytes.len()) {
        result.push(0);
    }

    result.append(&mut bytes);

    result
}
