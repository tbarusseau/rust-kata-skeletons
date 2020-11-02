use std::fs::read_dir;

fn main() -> Result<(), ()> {
    let r = read_dir("./src/bin").expect("Couldn't find ./src/bin directory");
    r.into_iter().for_each(|e| {
        println!("Usage: cargo run --bin <bin>");
        println!("Available binaries:");
        println!(
            "$ cargo run --bin {}",
            e.unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .strip_suffix(".rs")
                .unwrap()
        );
    });

    Ok(())
}
