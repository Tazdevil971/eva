fn main() {
    let config = eva_build::Config::load();

    if config.target_chip != "stm32f767zi" {
        println!(
            "cargo::error=eva-bsp only supports stm32f767zi! Please use target_chip=stm32f767zi!"
        );
        return;
    }

    let link_dir = std::env::current_dir()
        .expect("Failed to find current dir")
        .join("link");

    // Make the link script available to linkers
    println!("cargo::rustc-link-search={}", link_dir.display());
}
