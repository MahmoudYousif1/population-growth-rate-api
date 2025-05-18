use crate::app_state::model::AppState;
use crate::utils::models::Operator;
use actix_web::{dev::ServerHandle, web::Data};
use std::{env, time::Duration};
use tokio::{fs, task, time};

pub fn spawn_persistence_tasks(
    shared_state: Data<AppState>,
    interval_minutes: u64,
    server_handle: ServerHandle,
) {
    let path = env::var("COUNTRY_POPULATION_FILE_PATH").unwrap_or_else(|e| {
        let default_path = "./resources/country_population.json".to_string();
        log::warn!(
            "COUNTRY_POPULATION_FILE_PATH not set ({:?}), defaulting to {}",
            e,
            default_path
        );
        default_path
    });

    {
        let state = shared_state.clone();
        let path = path.clone();
        task::spawn(async move {
            let mut ticker = time::interval(Duration::from_secs(interval_minutes * 60));
            loop {
                ticker.tick().await;
                if let Err(e) = save_to_disk(&state, &path).await {
                    log::error!("Periodic save failed: {}", e);
                }
            }
        });
    }

    {
        let state = shared_state.clone();
        let path = path.clone();
        task::spawn(async move {
            let _ = tokio::signal::ctrl_c().await;
            let _ = save_to_disk(&state, &path).await;
            server_handle.stop(true).await;
        });
    }
}

pub(crate) async fn save_to_disk(state: &AppState, file_path: &str) -> Result<(), std::io::Error> {
    let json = {
        let guard = state
            .operators
            .read()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Lock poisoned"))?;
        let operators_to_serialize: Vec<&Operator> = guard.iter().collect();
        serde_json::to_string_pretty(&operators_to_serialize).unwrap_or_else(|e| {
            log::error!("Failed to serialize country list: {}", e);
            "[]".into()
        })
    };

    let tmp = format!("{}.tmp", file_path);
    fs::write(&tmp, &json).await?;
    fs::rename(&tmp, file_path).await?;

    log::info!("State saved to {}", file_path);
    Ok(())
}
