// Functions to create:
// - `add_one`, which takes a number and adds one to it
// - `double_n_times`, which doubles a number `n` times
// - `multiply_by_pi`, which multiplies a number by the PI constant
// - Create a function that returns an array of `n` elements, whatever you want

use std::f64::consts::PI;

fn add_one(i: i32) -> i32 {
    i + 1
}

fn double_n_times(n: i32, p: u32) -> i32 {
    n.pow(p)
}

fn multiply_by_pi(n: f64) -> f64 {
    n * PI
}

fn arrayer() -> [i32; 3] {
    [1, 2, 3]
}

fn main() {
    let res = add_one(2);
    println!("add_one {}", res);

    let double = double_n_times(2, 2);
    println!("double_n_times {}", double);

    let pim = multiply_by_pi(2.0);
    println!("multiply_by_pi {}", pim);
    
    let array = arrayer();
    println!("arrayer {:?}", array);
}
