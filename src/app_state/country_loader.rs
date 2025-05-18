use crate::app_state::AppState;
use crate::utils::models::CountryRecord;
use log::{info, warn};
use std::{default, env};
use tokio::fs;

pub async fn load_operator_mappings() -> Result<AppState, String> {
    let path = env::var("COUNTRY_POPULATION_FILE_PATH").unwrap_or_else(|e| {
        let default = "./resources/country_population.json".to_string();
        warn!(
            "COUNTRY_POPULATION_FILE_PATH not set ({:?}), defaulting to {}",
            e, default
        );
        default
    });

    let data = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;

    let list: Vec<Operator> =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    info!("Loaded {} operators from {}", list.len(), path);
    Ok(AppState::new(list, path))
}
