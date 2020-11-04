// Seen in this chapter:
// - Ownership
// - Scopes (and dropping)
// - Ownership moving, borrowing, and mutably borrowing
// - Dereferencing
// - Lifetimes

// Exercise: create a function that increments a number, then print it in main. Does it work?
// Exercise: create a function that adds a suffix to a String, then print it in main. Does it work?
// Exercise: if the previous function didn't work, go and fix it!

// Exercise: create a variable and drop it, both implicitly and explicitly

// Exercise: create a struct that contains at least one reference

fn increment(a: i32) -> i32 {
    unimplemented!()
}

fn suffix(s: String) -> String {
    unimplemented!()
}

fn main() {
    let a = 0;
    increment(a);

    // let s = "theTribe".to_string();
    // suffix(s);
    // println!("{}", s);
}
