// Functions to create:
// - `add_one`, which takes a number and adds one to it
// - `double_n_times`, which doubles a number `n` times
// - `multiply_by_pi`, which multiplies a number by the PI constant
// - Create a function that returns an array of `n` elements, whatever you want

use std::f64::consts::PI;

fn add_one(x: i32) -> i32 {
    return x + 1;
}

fn double_n_times(x: i32, n: u32) -> i32 {
    let base = 2i32;
    return x * base.pow(n);
}

fn multiply_by_pi(x: f64) -> f64 {
    return PI * x;
}

fn trois_array() -> [i32; 3] {
    return [1, 2, 3];
}

fn main() {
    println!("{}, {}, {}, {:?}", add_one(12), double_n_times(12, 3), multiply_by_pi(12f64), trois_array());
}
