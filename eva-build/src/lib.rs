use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub target_chip: String,
}

impl Config {
    pub fn load() -> Self {
        let target_chip =
            env::var("DEP_EVA_CONFIG_TARGET_CHIP").expect("eva-config is not a dependency!");
        Self { target_chip }
    }
}

#[doc(hidden)]
pub mod __eva_config {
    pub fn set_target_chip(target_chip: &str) {
        println!("cargo::metadata=target_chip={target_chip}");
    }
}
