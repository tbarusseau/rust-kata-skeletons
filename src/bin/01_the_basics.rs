// Functions to create:
// - `add_one`, which takes a number and adds one to it
// - `double_n_times`, which doubles a number `n` times
// - `multiply_by_pi`, which multiplies a number by the PI constant
// - Create a function that returns an array of `n` elements, whatever you want

// use std::f64::consts::PI;

fn add_one(number: i32) -> i32 {
    return number + 1;
}

fn double_n_times(number: i32, power: u32) -> i32 {
    return number.pow(power);
}

fn main() {
    print!("{}", add_one(12));
    print!("\n{}", double_n_times(4, 4));
}
