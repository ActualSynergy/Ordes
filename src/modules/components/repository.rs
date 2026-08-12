// Data access layer for components

use super::domain::Component;

#[async_trait::async_trait]
pub trait ComponentRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Component>, String>;
    async fn save(&self, component: &Component) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn list_all(&self, limit: i32, offset: i32) -> Result<Vec<Component>, String>;
    async fn find_by_type(&self, component_type: &str) -> Result<Vec<Component>, String>;
}

pub struct InMemoryComponentRepository {
    data: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Component>>>,
}

impl InMemoryComponentRepository {
    pub fn new() -> Self {
        Self {
            data: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl ComponentRepository for InMemoryComponentRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Component>, String> {
        Ok(self.data.lock().unwrap().get(id).cloned())
    }

    async fn save(&self, component: &Component) -> Result<(), String> {
        self.data.lock().unwrap().insert(component.id.clone(), component.clone());
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.data.lock().unwrap().remove(id);
        Ok(())
    }

    async fn list_all(&self, limit: i32, _offset: i32) -> Result<Vec<Component>, String> {
        Ok(self.data.lock().unwrap().values().take(limit as usize).cloned().collect())
    }

    async fn find_by_type(&self, _component_type: &str) -> Result<Vec<Component>, String> {
        Ok(self.data.lock().unwrap().values().cloned().collect())
    }
}
