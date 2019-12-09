use fnv::FnvHasher;
use fxhash::FxHasher;
use siphasher::sip::SipHasher24;
use std::hash::Hasher;
use std::time::Instant;

macro_rules! hash_bench {
    ($h: ty, $m: tt, $t: ty) => {
        let now = Instant::now();
        let mut n = 0;
        for i in 0..1000 * 1000 * 1000 {
            let mut hasher = <$h>::default();
            hasher.$m(i as $t);
            n += hasher.finish();
        }
        println!(
            "{}.{}: {:?} {}",
            stringify!($h),
            stringify!($m),
            Instant::now().duration_since(now),
            n
        );
    };
}

fn main() {
    hash_bench!(FnvHasher, write_u8, u8);
    hash_bench!(FnvHasher, write_u16, u16);
    hash_bench!(FnvHasher, write_u32, u32);
    hash_bench!(FnvHasher, write_u64, u64);
    hash_bench!(FnvHasher, write_u128, u128);

    hash_bench!(FxHasher, write_u8, u8);
    hash_bench!(FxHasher, write_u16, u16);
    hash_bench!(FxHasher, write_u32, u32);
    hash_bench!(FxHasher, write_u64, u64);
    hash_bench!(FxHasher, write_u128, u128);

    hash_bench!(SipHasher24, write_u8, u8);
    hash_bench!(SipHasher24, write_u16, u16);
    hash_bench!(SipHasher24, write_u32, u32);
    hash_bench!(SipHasher24, write_u64, u64);
    hash_bench!(SipHasher24, write_u128, u128);
}
