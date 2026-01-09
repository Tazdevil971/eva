fn main() {
    let link_dir = std::env::current_dir()
        .expect("Failed to find current dir")
        .join("link");

    // Make the link script available to linkers
    println!("cargo::rustc-link-search={}", link_dir.display());
}
