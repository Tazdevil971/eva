fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target != "thumbv7em-eva-eabihf" {
        println!("cargo::warning=Only thumbv7em-eva-eabihf is supported at the moment");
        panic!("Unsupported target");
    }
    
    println!("cargo::rerun-if-changed=src/stub.c");
    
    cc::Build::new()
        .flag("-fno-stack-protector")
        .include(std::env::var_os("DEP_EVA_INCLUDE").unwrap())
        .file("src/stub.c")
        .compile("libc-stub");
    
    println!("cargo::rustc-link-search=/usr/arm-none-eabi/lib/thumb/v7e-m+fp/hard/");
    println!("cargo::rustc-link-lib=c");
}
