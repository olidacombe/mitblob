use std::sync::OnceLock;

use derive_builder::Builder;

#[derive(Debug, Builder)]
#[builder(derive(serde::Deserialize))]
pub struct Config {
    #[builder(default = "\"main\".to_string()")]
    pub git_branch: String,
    pub git_repo: String,
    #[builder(default = "3000")]
    pub port: u16,
}

/// Gets app config from config files and the environment
pub fn get() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(|| {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix(
                &std::env!("CARGO_PKG_NAME").to_uppercase(),
            ))
            .build()
            .unwrap();

        let config_builder: ConfigBuilder = config.try_deserialize().unwrap();
        config_builder.build().unwrap()
    })
}
