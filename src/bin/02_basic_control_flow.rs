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

fn fizzbuzzWithIf() {
    for x in 0..=100 {
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz");
        }
        else if x % 3 == 0 {
            println!("Fizz");
        }
        else if x % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", x);
        }
    }
}

fn fizzbuzz() {
    for x in 0..=100 {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

fn main() {
    fizzbuzz();
}
