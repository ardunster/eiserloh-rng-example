# Noise Based Seeded RNG Functions

These functions are based on examples taken from [Squirrel Eiserloh](http://www.eiserloh.net/bio/)'s GDC talk, [Noise-Based RNG](https://youtu.be/LWFzPP8ZbdU?si=SYEybV8-AiFmoVf2). The provided examples are in C++, but here they are implemented in Rust.

Of course, you can always just use [`std::hash`](https://doc.rust-lang.org/std/hash/index.html) and save yourself some time, but seeing how these kinds of things work is an interesting exercise!

For more details, please see my [blog post](https://www.annardunster.com/posts/2024/eiserloh-noise-rng-rust) discussing the contents.
