use std::hash::{BuildHasher, Hasher};
use rand::{Rng, thread_rng};

/**
    A faster hasher, specialized for hashing u64s.
    It *can* be used with other sizes, but this comes with degraded hash quality
    (the most significant bits will be constant if the type is smaller than u64, for instance).
**/
pub struct IdentityHashBuilder {
    seed: u64,
}

impl IdentityHashBuilder {
    pub fn new() -> Self {
        IdentityHashBuilder {
            seed: thread_rng().gen::<u64>()
        }
    }
}

impl BuildHasher for IdentityHashBuilder {
    type Hasher = IdentityHash;

    #[inline]
    fn build_hasher(&self) -> IdentityHash {
        IdentityHash {
            seed: self.seed,
            state: 0,
        }
    }
}

pub struct IdentityHash {
    seed: u64,
    state: u64
}

impl Hasher for IdentityHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.state = (self.state << 8) | *b as u64;
        }
    }

    fn finish(&self) -> u64 {
        self.state ^ self.seed
    }
}
