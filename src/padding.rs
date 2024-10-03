use num_bigint::{BigUint, RandBigInt};
use num_traits::ToBytes;
use rand::{random, thread_rng};
use crate::sha2::sha512;

pub fn encode(chunk: &[u8], label: &[u8]) -> Vec<u8> {
    let rsa_len = 1024 / 8;
    let hash_len = 256 / 8;
    assert!(chunk.len() <= rsa_len - 2 * hash_len - 2);

    let mut label_hash = sha512(label).iter().flat_map(|x| x.to_be_bytes()).collect::<Vec<_>>();
    label_hash.truncate(hash_len);

    let mut db = Vec::with_capacity(rsa_len - hash_len - 1);

    db.extend_from_slice(&label_hash);
    for _ in 0..(rsa_len - chunk.len() - 2 * hash_len - 2) {
        db.push(0);
    }
    db.push(1);
    db.extend_from_slice(chunk);

    let mut seed = thread_rng().gen_biguint((hash_len * 8) as u64).to_be_bytes();
    if seed.len() < hash_len {
        seed.insert(0, 0);
    }

    let db_mask = mgf1(&seed, db.len());

    let mut masked_db = db.iter().zip(db_mask.iter()).map(|(x, y)| x ^ y).collect::<Vec<_>>();
    let seed_mask = mgf1(&masked_db, hash_len);
    let mut masked_seed = seed.iter().zip(seed_mask.iter()).map(|(x, y)| x ^ y).collect::<Vec<_>>();

    let mut encoded = Vec::with_capacity(rsa_len);

    encoded.push(0);
    encoded.append(&mut masked_seed);
    encoded.append(&mut masked_db);

    encoded
}

pub fn decode(message: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let message = &message[1..];
    let hash_len = 256 / 8;
    let masked_seed = &message[..hash_len];
    let masked_db = &message[hash_len..];

    let seed_mask = mgf1(&masked_db, hash_len);
    let seed = masked_seed.iter().zip(seed_mask.iter()).map(|(x, y)| x ^ y).collect::<Vec<_>>();

    let db_mask = mgf1(&seed, masked_db.len());
    let db = masked_db.iter().zip(db_mask.iter()).map(|(x, y)| x ^ y).collect::<Vec<_>>();

    let label_hash = &db[0..hash_len];

    let mut message_start = hash_len - 1;

    while db[message_start] != 1 {
        message_start += 1;
    }
    message_start += 1;


    (db[message_start..].to_vec(), label_hash.to_vec())
}


fn mgf1(seed: &[u8], length: usize) -> Vec<u8> {
    let hash_len = 512;

    assert!(length < (hash_len << 32));

    let mut bytes = Vec::with_capacity(length);

    let mut counter: u32 = 0;
    while bytes.len() < length {
        let c = counter.to_be_bytes();
        bytes.append(&mut sha512(&[seed, c.as_slice()].concat()).iter().flat_map(|x| x.to_be_bytes()).collect::<Vec<_>>());
        counter += 1;
    }

    bytes.truncate(length);

    bytes
}