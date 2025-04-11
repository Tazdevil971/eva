mod stm32 {
    include!("src/stm32/list.rs");
}

fn main() {
    println!("cargo::rerun-if-env-changed=EVA_TARGET_CHIP");

    let Ok(target_chip) = std::env::var("EVA_TARGET_CHIP") else {
        println!("cargo::error=eva-config failure, EVA_TARGET_CHIP is not set!");
        return;
    };

    println!("cargo::rustc-check-cfg=cfg(target_chip_family, values(\"stm32\", \"host\"))");

    if stm32::CHIPS_LIST.contains(&target_chip.as_str()) {
        // The target chip is from STM32
        println!("cargo::rustc-cfg=target_chip=\"{target_chip}\"");
        println!("cargo::rustc-cfg=target_chip_family=\"stm32\"");
        println!(
            "cargo::rustc-check-cfg=cfg(target_chip, values({}))",
            stm32::CHIPS_LIST.map(|s| format!("\"{s}\"")).join(",")
        );
    } else if target_chip == "host" {
        println!("cargo::rustc-cfg=target_chip_family=\"host\"");
    } else {
        println!(
            "cargo::error=The selected target_chip ({target_chip}) is not supported by eva-pac"
        );
    }
}
