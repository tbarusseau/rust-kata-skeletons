// Seen in this chapter:
// - Generic types in structs, enums
// - Option :D
// - Result :D!
// - Error handling?!
// - Unwrapping
// - Vectors

// That's a lot!

// Exercise: create a generic struct that holds a type,
// and an enum that contains several different generic types.
// And... That's it. We're still lacking important stuff to write
// interesting code with generic types.

struct _BagOfHolding<T> {
    content: T,
}

enum _BagOrOther<T, U>  {
    Bag(_BagOfHolding<T>),
    Other(U),
}

const SUFFIX: &str = "- Copy";

fn suffix_remover_1(filename: &String) -> Result<String, String> {
    if filename.ends_with(SUFFIX) {
        Ok(filename.split(SUFFIX).collect())
    }
    else {
        Err(format!("Filename does not end with suffix: `{}`", SUFFIX))
    }
}

fn _suffix_remover_2(filename: String) -> Result<String, ()> {
    match filename.strip_suffix(SUFFIX) {
        Some(s) => Ok(s.to_string()),
        None => Err(()),
    }
}

fn _suffix_remover_3(filename: String) -> Result<String, ()> {
    if filename.ends_with(SUFFIX) {
        let len = filename.chars().count();
        Ok(filename.chars().take(len - SUFFIX.len()).collect())
    }
    else {
        Err(())
    }
}

fn _suffix_remover_4(filename: String) -> Result<String, ()> {
    if filename.ends_with(SUFFIX) {
        let len = filename.chars().count();
        Ok(filename[..len - SUFFIX.len()].to_string())
    }
    else {
        Err(())
    }
}

// #[test]
// fn test_suffix_remover() {
//     const input: &str = "test- Copy";
//     const output: &str = "test";
// }

// Exercise: use the previous function, and print the result after being unwrapped!

// Exercise: use the previous function, print the result after using the `?` operator
// from the `main` function. It means that we'll need to update the `main` function's prototype!

// Exercise: use the previous function, and print an error message if the function fails.

fn main() -> Result<(), String> {
    let s = "test".to_string();
    let result = suffix_remover_1(&s)?;
    println!("{}", result);
    println!("{}", s);

    Ok(())
}
