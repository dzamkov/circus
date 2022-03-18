use crate::*;
use array_init::array_init;

/// Encapsulates the state of a SHA-256 hasher at a certain point in its input.
pub struct Sha256([u32; 8]);

impl Sha256 {
    /// Constructs a SHA-256 hasher in its initial state.
    pub fn new() -> Self {
        Self(H)
    }
}

impl std::fmt::Display for Sha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7]
        )
    }
}

impl<S: SystemRepr<u32> + ?Sized> SystemRepr<Sha256> for S {
    type Abstract = [Abstract<Self, u32>; 8];
    fn constant(&mut self, value: Sha256) -> Self::Abstract {
        value.0.map(|h| self.constant(h))
    }
}

/// A system in which SHA-256 hashes can be computed.
pub trait SystemSha256:
    SystemRepr<u32>
    + SystemWrappingAdd<u32>
    + SystemBitAnd<u32>
    + SystemBitXor<u32>
    + SystemBitRotate<u32, u8>
    + SystemBitShift<u32, u8>
    + SystemNot<u32>
{
    /// Initializes a SHA-256 hasher.
    fn sha256_new(&mut self) -> Abstract<Self, Sha256> {
        <Self as SystemRepr<Sha256>>::constant(self, Sha256::new())
    }

    /// Updates a SHA-256 hasher with the next chunk of data.
    fn sha256_update(&mut self, hasher: &mut Abstract<Self, Sha256>, chunk: [u32; 16]) {
        // Initialize message schedule
        let mut w: [_; 64] = array_init(|_| self.constant(0));
        for i in 0..16 {
            w[i] = self.constant(chunk[i]);
        }
        for i in 16..64 {
            let s0 = &w[i - 15];
            let t0 = self.rotr(s0, 7);
            let t1 = self.rotr(s0, 18);
            let t2 = self.shr(s0, 3);
            let s0 = self.xor(&t0, &t1);
            let s0 = self.xor(&s0, &t2);
            let s1 = &w[i - 2];
            let t0 = self.rotr(s1, 17);
            let t1 = self.rotr(s1, 19);
            let t2 = self.shr(s1, 10);
            let s1 = self.xor(&t0, &t1);
            let s1 = self.xor(&s1, &t2);
            let r = self.wrapping_add(&w[i - 16], &s0);
            let r = self.wrapping_add(&r, &w[i - 7]);
            let r = self.wrapping_add(&r, &s1);
            w[i] = r;
        }

        // Initialize working variables
        let mut a = hasher[0].clone();
        let mut b = hasher[1].clone();
        let mut c = hasher[2].clone();
        let mut d = hasher[3].clone();
        let mut e = hasher[4].clone();
        let mut f = hasher[5].clone();
        let mut g = hasher[6].clone();
        let mut h = hasher[7].clone();

        // Compression function main loop
        for i in 0..64 {
            let t0 = self.rotr(&e, 6);
            let t1 = self.rotr(&e, 11);
            let t2 = self.rotr(&e, 25);
            let s1 = self.xor(&t0, &t1);
            let s1 = self.xor(&s1, &t2);
            let t0 = self.and(&e, &f);
            let t1 = self.not(&e);
            let t2 = self.and(&t1, &g);
            let ch = self.xor(&t0, &t2);
            let k = self.constant(K[i]);
            let temp1 = self.wrapping_add(&h, &s1);
            let temp1 = self.wrapping_add(&temp1, &ch);
            let temp1 = self.wrapping_add(&temp1, &k);
            let temp1 = self.wrapping_add(&temp1, &w[i]);
            let t0 = self.rotr(&a, 2);
            let t1 = self.rotr(&a, 13);
            let t2 = self.rotr(&a, 22);
            let s0 = self.xor(&t0, &t1);
            let s0 = self.xor(&s0, &t2);
            let t0 = self.and(&a, &b);
            let t1 = self.and(&a, &c);
            let t2 = self.and(&b, &c);
            let maj = self.xor(&t0, &t1);
            let maj = self.xor(&maj, &t2);
            let temp2 = self.wrapping_add(&s0, &maj);
            h = g;
            g = f;
            f = e;
            e = self.wrapping_add(&d, &temp1);
            d = c;
            c = b;
            b = a;
            a = self.wrapping_add(&temp1, &temp2);
        }

        // Add to current hash value
        hasher[0] = self.wrapping_add(&hasher[0], &a);
        hasher[1] = self.wrapping_add(&hasher[1], &b);
        hasher[2] = self.wrapping_add(&hasher[2], &c);
        hasher[3] = self.wrapping_add(&hasher[3], &d);
        hasher[4] = self.wrapping_add(&hasher[4], &e);
        hasher[5] = self.wrapping_add(&hasher[5], &f);
        hasher[6] = self.wrapping_add(&hasher[6], &g);
        hasher[7] = self.wrapping_add(&hasher[7], &h);
    }
}

impl<
        S: SystemRepr<u32>
            + SystemWrappingAdd<u32>
            + SystemBitAnd<u32>
            + SystemBitXor<u32>
            + SystemBitRotate<u32, u8>
            + SystemBitShift<u32, u8>
            + SystemNot<u32>,
    > SystemSha256 for S
{
}

const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[test]
fn test_empty() {
    let mut hasher = Sha256::new();
    Eval.sha256_update(&mut hasher.0, [0x80000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let target = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    assert_eq!(format!("{}", hasher), target);
}
