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

fn increment(a: &mut i32) {
    *a += 1;
}

fn suffix(s: &mut String, f: &str) {
    s.push_str(&f);
}

fn main() {
    let mut a = 0;
    increment(&mut a);
    println!("{}", a);
    
    let mut s = "theTribe".to_string();
    suffix(&mut s, " toto");
    println!("{}", s);

    let mut s = "hello".to_string();
    suffix(&mut s, " rust world");
    println!("{}", s);
}
