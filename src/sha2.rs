pub fn sum<const N: usize>(nums: [u64; N]) -> u64 {
    let mut sum: u64 = 0;
    for num in nums {
        sum = sum.overflowing_add(num).0;
    }

    return sum;
}


pub fn sha512(message: &[u8]) -> [u64; 8] {
    let mut hash_values = INITIAL_HASH_VALUES_512.clone();

    let mut padded = message.to_vec();
    padded.push(0x80);

    let padding_length = (128 - ((message.len() + 1 + 16) % 128)) % 128;
    for _ in 0..padding_length {
        padded.push(0);
    }

    padded.extend_from_slice(&(message.len() as u128).to_be_bytes());

    let chunks = padded.chunks(128);

    for chunk in chunks {
        let chunk_u64 = chunk.chunks(8).map(|x| u64::from_be_bytes(x.try_into().unwrap())).collect::<Vec<_>>();

        let mut w = [0u64; 80];
        w[0..16].copy_from_slice(chunk_u64.as_slice());

        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = sum([w[i - 16], s0, w[i - 7], s1]);
        }

        let mut a = hash_values[0];
        let mut b = hash_values[1];
        let mut c = hash_values[2];
        let mut d = hash_values[3];
        let mut e = hash_values[4];
        let mut f = hash_values[5];
        let mut g = hash_values[6];
        let mut h = hash_values[7];


        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let temp1 = sum([h, s1, ch, ROUND_CONSTANTS_512[i], w[i]]);

            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.overflowing_add(maj).0;

            h = g;
            g = f;
            f = e;
            e = d.overflowing_add(temp1).0;
            d = c;
            c = b;
            b = a;
            a = temp1.overflowing_add(temp2).0;
        }

        hash_values[0] = hash_values[0].overflowing_add(a).0;
        hash_values[1] = hash_values[1].overflowing_add(b).0;
        hash_values[2] = hash_values[2].overflowing_add(c).0;
        hash_values[3] = hash_values[3].overflowing_add(d).0;
        hash_values[4] = hash_values[4].overflowing_add(e).0;
        hash_values[5] = hash_values[5].overflowing_add(f).0;
        hash_values[6] = hash_values[6].overflowing_add(g).0;
        hash_values[7] = hash_values[7].overflowing_add(h).0;
    }

    hash_values
}


const INITIAL_HASH_VALUES_512: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const ROUND_CONSTANTS_512: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];