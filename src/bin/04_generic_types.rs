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
struct Container<T> {
    item: T
}

// enum Cargo<T, S> {
//     Place = Container(String),
    
// }

fn remove_copy(filename: String) -> Result<String, String> {
    if filename.ends_with("- Copy") {
        Ok(filename.split("- Copy").collect())
    }
    else {
        Err("No '- Copy' found".to_string())
    }

}

fn main() -> Result<(), String> {
    let result = remove_copy("Mon fichier - Copy".to_string())?;

    println!("{}", result);

    Ok(())
}
