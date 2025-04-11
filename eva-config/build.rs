use std::env;

fn main() {
    println!("cargo::rerun-if-env-changed=EVA_TARGET_CHIP");

    let Ok(target_chip) = env::var("EVA_TARGET_CHIP") else {
        println!("cargo::error=eva-config failure, EVA_TARGET_CHIP is not set!");
        return;
    };

    eva_build::__eva_config::set_target_chip(&target_chip);
}
