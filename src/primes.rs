use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{One, Zero};
use rand::Rng;

pub fn rand_prime(rng: &mut impl Rng, bits: usize) -> BigUint {
    loop {
        let candidate = rng.gen_biguint(bits as u64);

        if is_probably_a_prime(rng, &candidate) {
            return candidate;
        }
    }
}

pub fn is_probably_a_prime(rng: &mut impl Rng, candidate: &BigUint) -> bool {
    if candidate < &2.to_biguint().unwrap() {
        return false;
    }

    if candidate % &2.to_biguint().unwrap() == BigUint::zero() {
        return false;
    }

    if candidate % &3.to_biguint().unwrap() == BigUint::zero() {
        return false;
    }

    // Test Millera-Rabina (50 iteracji)
    for _ in 0..50 {
        let a = rng.gen_biguint_range(&BigUint::from(2_u32), candidate);
        if a.modpow(&(candidate - 1_u32), candidate) != BigUint::one() {
            return false;
        }
    }

    true
}
