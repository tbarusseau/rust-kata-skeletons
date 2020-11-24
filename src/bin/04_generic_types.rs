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

use regex::Regex;

struct _Bag<T> {
    content: T,
}

enum _BagOrOther<T, U> {
    Bag(_Bag<T>),
    Other(U),
}

fn dot_txt_remover(file_name: &str)-> Result<String, String> {
    if file_name.contains(".txt") {
        let reg = Regex::new(r".txt$").unwrap();

        Ok(reg.replace(file_name, "").to_string())
    } else { 
        Err("Not found".to_string()) 
    }
}

fn main() {
    let result: Result<String, String> = dot_txt_remover("myFile.txt");
    match result {
        Ok(splitted_file_name) => println!("file name without extension is : {}", splitted_file_name),
        Err(e) => println!("an error occurs : {}", e),
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn should_output_the_without_extenstion(){
        assert_eq!(dot_txt_remover("myFile.txt").unwrap(), String::from("myFile"));
        assert_eq!(dot_txt_remover("myFile.txt.txt").unwrap(), String::from("myFile.txt"));
        assert_eq!(dot_txt_remover("myFile.txt.io").unwrap(), String::from("myFile.txt.io"));
    }
    #[test]
    fn should_output_an_error(){
        assert_eq!(dot_txt_remover("myFile.io"), Err("Not found".to_string()));
        assert_eq!(dot_txt_remover("myFile"), Err("Not found".to_string()));
        assert_eq!(dot_txt_remover("myFiletxt.something"), Err("Not found".to_string()));
    }
}