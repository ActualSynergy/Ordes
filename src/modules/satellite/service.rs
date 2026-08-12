// Business logic layer for satellite operations

use super::domain::Satellite;
use super::repository::SatelliteRepository;

pub struct SatelliteService {
    repository: std::sync::Arc<dyn SatelliteRepository>,
}

impl SatelliteService {
    pub fn new(repository: std::sync::Arc<dyn SatelliteRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_satellite(&self, name: String, description: String) -> Result<Satellite, String> {
        let satellite = Satellite::new(name, description);
        self.repository.save(&satellite).await?;
        Ok(satellite)
    }

    pub async fn get_satellite(&self, id: &str) -> Result<Option<Satellite>, String> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_satellite(&self, satellite: Satellite) -> Result<(), String> {
        self.repository.save(&satellite).await
    }

    pub async fn delete_satellite(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id).await
    }
}
