// Business logic layer for components

use super::domain::{Component, ComponentType};
use super::repository::ComponentRepository;

pub struct ComponentService {
    repository: std::sync::Arc<dyn ComponentRepository>,
}

impl ComponentService {
    pub fn new(repository: std::sync::Arc<dyn ComponentRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_component(
        &self,
        name: String,
        component_type: ComponentType,
        mass: f64,
        created_by: String,
    ) -> Result<Component, String> {
        let component = Component::new(name, component_type, mass, created_by);
        self.repository.save(&component).await?;
        Ok(component)
    }

    pub async fn get_component(&self, id: &str) -> Result<Option<Component>, String> {
        self.repository.find_by_id(id).await
    }

    pub async fn list_components(&self, limit: i32, offset: i32) -> Result<Vec<Component>, String> {
        self.repository.list_all(limit, offset).await
    }
}
