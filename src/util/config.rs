use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub index_file: String,
}

pub fn boot() -> Config {
    let current_dir = env::current_dir().expect("Gagal mendapatkan direktori saat ini");
    let config_file = current_dir.join("ruber.yaml");
    let config = fs::read_to_string(config_file).expect("Gagal membaca file konfigurasi");
    let mut config: Config = serde_yaml::from_str(&config).unwrap();
    config.index_file = current_dir
        .join(config.index_file)
        .to_str()
        .unwrap()
        .to_string()
        + ".rub";
    config
}
