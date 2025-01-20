use std::sync::OnceLock;

static CONFIG_PATH: OnceLock<String> = OnceLock::new();

// pub fn app_config_path() -> String {
//     CONFIG_PATH.get().unwrap().to_string()
// }

pub fn initialize(config_path: &str) {
    CONFIG_PATH.get_or_init(|| config_path.to_string());
}



