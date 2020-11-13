// Functions to create:
// - `add_one`, which takes a number and adds one to it
// - `double_n_times`, which doubles a number `n` times
// - `multiply_by_pi`, which multiplies a number by the PI constant
// - Create a function that returns an array of `n` elements, whatever you want

use std::f64::consts::PI;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn double_n_times(x: i32, n: u32) -> i32 {
    x.pow(n)
}

fn multiply_by_pi(x: f64) -> f64 {
    x * PI
}

fn three_elements_array() -> [i32; 3] {
    [-1, 10, 42]
}

fn main() {
    println!("{}", add_one(42));

    println!("{}", double_n_times(42, 3));

    println!("{}", multiply_by_pi(2f64));

    println!("{:?}", three_elements_array());
}
