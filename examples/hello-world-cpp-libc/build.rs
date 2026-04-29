fn main() {
    println!("cargo::rerun-if-changed=src/main.cpp");

    cc::Build::new()
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-fno-stack-protector")
        .include(std::env::var_os("DEP_EVA_INCLUDE").unwrap())
        .file("src/main.cpp")
        .compile("hello-world-cpp-libc");
    
    println!("cargo::rustc-link-search=/usr/lib/gcc/arm-none-eabi/9.3.0/");
    println!("cargo::rustc-link-lib=stdc++");
    println!("cargo::rustc-link-lib=gcc");
}
