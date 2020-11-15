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

fn compute(n: i32) -> String {
    if n == 0 { return "0".to_string() }
    match (n % 3, n % 5, n % 7) {
        (0, 0, _) => "FizzBuzz".to_string(),
        (_, 0, _) => "Buzz".to_string(),
        (0, _, _) => "Fizz".to_string(),
        (_, _, 0) => "Jazz".to_string(),
        _ => n.to_string(),
    }
}

fn fizzbuzz(start: i32, end: i32) -> impl Iterator<Item = String> {
    return (start..=end).map(|x| compute(x));
}

fn main() {
    fizzbuzz(0, 100).for_each(|x| {
        println!("{}", x);
    });
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    fn fizzbuzz_number_factory(n: i32) -> Option<std::string::String> {
        return fizzbuzz(n, n).next();
    }

    #[test]
    fn should_output_the_number_when_number_is_not_multiple_of_3_nor_5(){
        assert_eq!(fizzbuzz_number_factory(0), Some("0".to_string()));
        assert_eq!(fizzbuzz_number_factory(1), Some("1".to_string()));
        assert_eq!(fizzbuzz_number_factory(2), Some("2".to_string()));
    }

    #[test]
    fn should_output_fizz_when_number_is_multiple_of_3(){
        assert_eq!(fizzbuzz_number_factory(3), Some("Fizz".to_string()));
        assert_eq!(fizzbuzz_number_factory(6), Some("Fizz".to_string()));
        assert_eq!(fizzbuzz_number_factory(9), Some("Fizz".to_string()));
    }

    #[test]
    fn should_output_buzz_when_number_is_multiple_of_5(){
        assert_eq!(fizzbuzz_number_factory(5), Some("Buzz".to_string()));
        assert_eq!(fizzbuzz_number_factory(10), Some("Buzz".to_string()));
    }

    #[test]
    fn should_output_buzz_when_number_is_multiple_of_7(){
        assert_eq!(fizzbuzz_number_factory(7), Some("Jazz".to_string()));
    }

    #[test]
    fn should_output_fizzbuzz_when_number_is_multiple_of_3_and_5() {
        assert_eq!(fizzbuzz_number_factory(15), Some("FizzBuzz".to_string()));
        assert_eq!(fizzbuzz_number_factory(30), Some("FizzBuzz".to_string()));
        assert_eq!(fizzbuzz_number_factory(45), Some("FizzBuzz".to_string()));
    }

    #[test]
    fn should_output_the_sixteenth_0_1_2_fizz_4_buzz_fizz_7_8_fizz_buzz_11_fizz_13_14_fizz_buzz() {
        let result: Vec<String> = fizzbuzz(0, 15).collect();
        let expected = [
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "Jazz".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "Jazz".to_string(),
            "FizzBuzz".to_string(),
        ];

        assert_eq!(result, expected);
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn should_create_a_list_of_n_elements_including_the_end_pad() {
        let result: Vec<String> = fizzbuzz(0, 100).collect();
        let expected: Vec<_> = (0..=100).collect();
        
        assert_eq!(result.len(), expected.len());
    }
}
