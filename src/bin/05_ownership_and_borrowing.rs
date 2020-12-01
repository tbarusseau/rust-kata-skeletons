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

fn suffix(s: &mut String) {
    s.push_str("-suffix");
}

fn main() {
    let mut a = 0;
    increment(&mut a);
    println!("{}", a);

    let mut s = "theTribe".to_string();
    suffix(&mut s);
    println!("{}", s);

    { let _drop_me = 128; }
    let drop_me_too = 42;
    std::mem::drop(drop_me_too);

    // println!("{}", _drop_me);
    println!("{}", drop_me_too);

    let drop_me_for_real_this_time = String::new();
    std::mem::drop(drop_me_for_real_this_time);
    // println!("{}", drop_me_for_real_this_time);

    let ref_holder: HoldsARef<str> = HoldsARef {
        my_ref: "toto",
    };
}

struct HoldsARef<'a, T: ?Sized> {
    my_ref: &'a T,
}
