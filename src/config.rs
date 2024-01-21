use std::fs;

const CONFIG_DIR: &'static str = "linefetch";
const CONFIG_FILE: &'static str = "config.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub modules: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        fn create_config_file(path: &str) -> Config {
            let config = Config::default();

            fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();

            config
        }

        let xdg_dirs = xdg::BaseDirectories::with_prefix(CONFIG_DIR).unwrap();

        let config_file = xdg_dirs.place_config_file(CONFIG_FILE).unwrap();

        if !config_file.exists() {
            return create_config_file(&config_file.to_str().unwrap());
        }

        let config_json = fs::read_to_string(&config_file).unwrap();

        serde_json::from_str(&config_json).unwrap_or_else(|_| {
            eprintln!("Corrupt config file found! Replacing it with a new one.");

            return create_config_file(&config_file.to_str().unwrap());
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            modules: vec![
                String::from("kernel"),
                String::from("ip-address"),
                String::from("memory")
            ],
        }
    }
}
