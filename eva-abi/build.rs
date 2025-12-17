fn main() {
    println!(
        "cargo::metadata=include={}",
        std::env::current_dir()
            .expect("Failed to get current path")
            .join("include")
            .display()
    );
}
