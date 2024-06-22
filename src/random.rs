use std::cell::RefCell;

thread_local! {
    /// A thread-local random number generator.
    pub static THREAD_RNG: RefCell<Random> = RefCell::new(Random::new(0xdeadbeef));
}

#[derive(Debug)]
/// A random number generator.
pub struct Random {
    /// The state of the random number generator.
    state: u64,
}

impl Random {
    /// Create a new random number generator with the given seed.
    pub const fn new(seed: u64) -> Self {
        assert!(seed != 0, "seed must not be zero");

        Self {
            state: Self::jenkins_hash(seed),
        }
    }

    /// Applies a slighly modified version of the "One At A Time"
    /// hash function to the input.
    /// See https://www.burtleburtle.net/bob/hash/doobs.html.
    const fn jenkins_hash(mut input: u64) -> u64 {
        input += input << 10;
        input ^= input >> 6;
        input += input << 3;
        input ^= input >> 11;
        input += input << 15;
        input
    }

    /// Applies the "xor" function from Marsaglia G., "Xorshift RNGs", Section 3.
    fn xor_shift64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Returns a random 64-bit floating point number in the range [0, 1].
    /// This sets the exponent to zero and sets the 52 most significant bits
    /// of a random 64 bit integer as the mantissa, this generates a
    /// number from [1.0, 1.9999999] which is then mapped to [0, 0.999999]
    /// by subtracting one. See Ray Tracing Gems II, Section 14.3.4.
    pub fn random_f64(&mut self) -> f64 {
        let rand = self.xor_shift64();
        let bits = 0x3ff0000000000000 | (rand >> 12);
        f64::from_bits(bits) - 1.0
    }
}
