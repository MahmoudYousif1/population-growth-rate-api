use crate::utils::models::CountryRecord;
use std::sync::RwLock;

pub struct AppState {
    pub country_records: RwLock<Vec<CountryRecord>>,
    pub mappings_file_path: String,
}

impl AppState {
    pub fn new(initial: Vec<CountryRecord>, mappings_file_path: String) -> Self {
        AppState {
            country_records: RwLock::new(initial),
            mappings_file_path,
        }
    }
}
