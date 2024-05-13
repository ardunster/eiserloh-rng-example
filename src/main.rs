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

    println!("Squirrel 3 results:");
    let seed = 12345; // Don't use this on your luggage.
    for number in 0..12 {
        println!(
            "Input {} with seed {} produces output {}",
            number,
            seed,
            squirrel_3(number, seed)
        )
    }
    let seed_two = 54321;
    for number in 0..12 {
        println!(
            "Input {} with seed {} produces output {}",
            number,
            seed_two,
            squirrel_3(number, seed_two)
        )
    }
}
