use std::fs::read_dir;

fn main() -> Result<(), ()> {
    let r = read_dir("./src/bin").expect("Couldn't find ./src/bin directory");

    println!("Usage: cargo run --bin <bin>");
    println!("Available binaries:");

    r.into_iter().for_each(|e| {
        let s = e.unwrap().file_name().into_string().unwrap();
        let s = match s.ends_with(".rs") {
            true => s.strip_suffix(".rs").unwrap().to_string(),
            _ => s,
        };

        println!("cargo run --bin {}", s);
    });

    Ok(())
}
