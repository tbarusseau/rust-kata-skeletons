// Functions to create:
// - `add_one`, which takes a number and adds one to it
// - `double_n_times`, which doubles a number `n` times
// - `multiply_by_pi`, which multiplies a number by the PI constant
// - Create a function that returns an array of `n` elements, whatever you want

use std::f64::consts::PI;

fn main() {
    println!("expect add_one(4) to be 5, result is : {}", add_one(4) == 4 + 1);
    println!("expect double_n_times(2, 3) to be 8, result is : {}", double_n_times(2, 3) == 2*2*2);
    println!("expect multiply_by_pi(2) to be ~3.28..., result is : {}", multiply_by_pi(2.0) == 2.0 * PI);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn double_n_times(x: i32, n: u32) -> i32 {
    x.pow(n)
}

fn multiply_by_pi(y: f64) -> f64 {
    y * PI
}

mod tests {
    #[cfg(test)]
    use super::*;
    
    #[test]
    fn should_increment_the_number_by_one() {
        assert_eq!(add_one(4), 4 + 1);
    }

    #[test]
    fn should_multiply_the_number_n_times() {
        assert_eq!(double_n_times(8, 3), 8 * 8 * 8);
    }

    #[test]
    fn should_multiply_the_number_by_pi_constant() {
        assert_eq!(multiply_by_pi(44.0), 44.0 * PI);
    }
}


