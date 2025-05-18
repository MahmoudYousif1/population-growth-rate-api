use log::warn;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub save_interval_minutes: u64,
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub country_population_file_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            save_interval_minutes: 60,
            host: "0.0.0.0".into(),
            port: 8080,
            workers: 4,
            country_population_file_path: "./resources/country_population.json".into(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let d = Config::default();

        fn parse_env_u64(key: &str, default: u64) -> u64 {
            match env::var(key) {
                Ok(s) => s.parse::<u64>().unwrap_or_else(|e| {
                    warn!("{}='{}' invalid ({:?}); default {}", key, s, e, default);
                    default
                }),
                Err(e) => {
                    warn!("{} not set ({:?}); default {}", key, e, default);
                    default
                }
            }
        }

        fn parse_env_usize(key: &str, default: usize) -> usize {
            match env::var(key) {
                Ok(s) => s.parse::<usize>().unwrap_or_else(|e| {
                    warn!("{}='{}' invalid ({:?}); default {}", key, s, e, default);
                    default
                }),
                Err(e) => {
                    warn!("{} not set ({:?}); default {}", key, e, default);
                    default
                }
            }
        }

        fn parse_env_u16(key: &str, default: u16) -> u16 {
            match env::var(key) {
                Ok(s) => s.parse::<u16>().unwrap_or_else(|e| {
                    warn!("{}='{}' invalid ({:?}); default {}", key, s, e, default);
                    default
                }),
                Err(e) => {
                    warn!("{} not set ({:?}); default {}", key, e, default);
                    default
                }
            }
        }

        fn parse_env_string(key: &str, default: &str) -> String {
            env::var(key).unwrap_or_else(|e| {
                warn!("{} not set ({:?}); default {}", key, e, default);
                default.to_string()
            })
        }

        Config {
            save_interval_minutes: parse_env_u64("SAVE_INTERVAL_MINUTES", d.save_interval_minutes),
            host: parse_env_string("HOST", &d.host),
            port: parse_env_u16("PORT", d.port),
            workers: parse_env_usize("WORKERS", d.workers),
            country_population_file_path: parse_env_string(
                "COUNTRY_POPULATION_FILE_PATH",
                &d.country_population_file_path,
            ),
        }
    }
}

pub fn load() -> Config {
    Config::from_env()
}
