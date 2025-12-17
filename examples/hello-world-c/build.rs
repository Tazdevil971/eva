fn main() {
    println!("cargo::rerun-if-changed=src/main.c");

    cc::Build::new()
        .target("x86_64-unknown-linux-gnu")
        .flag("-ffreestanding")
        .include(std::env::var_os("DEP_EVA_INCLUDE").unwrap())
        .file("src/main.c")
        .compile("hello-world-c");
}
