// Data access layer for satellite persistence

use super::domain::Satellite;

#[async_trait::async_trait]
pub trait SatelliteRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Satellite>, String>;
    async fn save(&self, satellite: &Satellite) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn list_by_project(&self, project_id: &str) -> Result<Vec<Satellite>, String>;
}

// In-memory implementation for development
pub struct InMemorySatelliteRepository {
    data: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Satellite>>>,
}

impl InMemorySatelliteRepository {
    pub fn new() -> Self {
        Self {
            data: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl SatelliteRepository for InMemorySatelliteRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Satellite>, String> {
        Ok(self.data.lock().unwrap().get(id).cloned())
    }

    async fn save(&self, satellite: &Satellite) -> Result<(), String> {
        self.data.lock().unwrap().insert(satellite.id.clone(), satellite.clone());
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.data.lock().unwrap().remove(id);
        Ok(())
    }

    async fn list_by_project(&self, _project_id: &str) -> Result<Vec<Satellite>, String> {
        Ok(self.data.lock().unwrap().values().cloned().collect())
    }
}
