/// The fourth [Bell Prime](https://en.wikipedia.org/wiki/Bell_number#Bell_primes), because Wikipedia.
const SOME_BIG_PRIME_NUMBER: u32 = 27_644_437;
/// The answer to every question that isn't 42.
const SOME_OTHER_NUMBER: u32 = 17;

pub fn some_noise_function(position: i32) -> u32 {
    let mut mangled = position as u32;
    mangled = mangled.wrapping_mul(SOME_BIG_PRIME_NUMBER);
    mangled = mangled.wrapping_add(SOME_OTHER_NUMBER);
    mangled = mangled.wrapping_mul(mangled);

    mangled ^= mangled >> 13;

    mangled
}

pub fn squirrel_3(position: i32, seed: u32) -> u32 {
    const BIT_NOISE1: u32 = 0xB5297A4D;
    const BIT_NOISE2: u32 = 0x68E31DA4;
    const BIT_NOISE3: u32 = 0x1B56C4E9;

    let mut mangled = position as u32;
    mangled *= BIT_NOISE1;
    mangled += seed;
    mangled ^= mangled >> 8;
    mangled += BIT_NOISE2;
    mangled ^= mangled << 8;
    mangled *= BIT_NOISE3;
    mangled ^= mangled >> 8;

    mangled
}
