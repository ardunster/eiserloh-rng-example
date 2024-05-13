use eiserloh_rng_example::*;

fn main() {
    println!("Hello, world!");
    let input = 2;
    let result = some_noise_function(input);
    println!("Got noise function result: {}", result);

    println!("Some Noise Function results:");
    for number in 0..12 {
        println!(
            "Input {} produces output {}",
            number,
            some_noise_function(number)
        )
    }
}
