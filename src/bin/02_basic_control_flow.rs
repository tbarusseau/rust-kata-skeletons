// Seen in the second chapter:
// - if/else if/else
// - loop, while, for
// - match

// This time, we'll create a FizzBuzz function:
// - Iterate over numbers from 0 to 100
// - If the number is divisible by 3, print `Fizz`.
// - If the number is divisible by 5, print `Buzz`.
//   This works with the previous rule if the number is both divisible by 3 and 5!
// - If the number is neither divisible by 3 nor 5, print the number.
// When it works, we'll create a FizzBuzzJazz function :D

fn fizzbuzz_if() {
    for x in 0..100 {
        if x % 5 == 0 && x % 3 == 0 {
            println!("{} {}", x, "FizzBuzz")
        } else if x % 3 == 0 {
            println!("{} {}", x, "Fizz")
        } else if x % 5 == 0 {
            println!("{} {}", x, "Buzz")
        } else {
            println!("{}", x)
        }
    }
}

fn fizzbuzz_match() {
    for x in 0..=100 {
        match (x % 3, x % 5) {
            (0, 0) => println!("{} {}", x, "FizzBuzz"),
            (0, _) => println!("{} {}", x, "Fizz"),
            (_, 0) => println!("{} {}", x, "Buzz"),
            _ => println!("{}", x),
        }
    }
}
fn fizzbuzzjazz() {
    for x in 0..=100 {
        match (x % 3, x % 5, x % 7) {
            (0, 0, 0) => println!("{} {}", x, "FizzBuzzJazz"),
            (0, _, _) => println!("{} {}", x, "Fizz"),
            (_, 0, _) => println!("{} {}", x, "Buzz"),
            (_, _, 0) => println!("{} {}", x, "Jazz"),
            _ => println!("{}", x),
        }
    }
}

fn main() {
    fizzbuzzjazz();
}
