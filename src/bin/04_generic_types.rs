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

// Exercise: create a function that takes a filename (`String`), removes a
// suffix (`- Copy`), and returns the filename if the suffix was found, else
// an error. Yes, we'll return a `Result` :D

// Exercise: use the previous function, and print the result after being unwrapped!

// Exercise: use the previous function, print the result after using the `?` operator
// from the `main` function. It means that we'll need to update the `main` function's prototype!

// Exercise: use the previous function, and print an error message if the function fails.

struct Generic<T> {
    item: T,
}

enum SeveralGenerics<T, U> {
    Bag(Generic<T>),
    Other(U)
}

fn remove_suffix(filename: &str) -> Result<String, String> {
    let suffix = "- Copy";
    if filename.ends_with(suffix) {
        return Ok(filename.split(suffix).collect::<String>());
    }
    Err(String::from("Impossible de supprimer le suffixe : suffixe non trouvé"))
}

fn remove_suffix_autrement(filename: &str) -> Result<String, String> {
    let suffix = "- Copy";
    if filename.ends_with(suffix) {
        return Ok(filename.trim_end_matches(suffix).to_string());
    }
    Err(String::from("Impossible de supprimer le suffixe : suffixe non trouvé"))
}

fn main() -> Result<(), String> {
    let success1 = remove_suffix("test - Copy")?;
    let success2 = remove_suffix_autrement("test - Copy")?;

    println!("{}", success1);
    println!("{}", success2);

    let failure1 = remove_suffix("test")?;
    let failure2 = remove_suffix_autrement("test")?;

    println!("{}", failure1);
    println!("{}", failure2);

    Ok(())
}
