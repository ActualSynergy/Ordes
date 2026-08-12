// Assembly service - Business logic for assembly operations

use super::domain::Assembly;
use super::repository::AssemblyRepository;
use crate::modules::components::ComponentInstance;

pub struct AssemblyService {
    repository: std::sync::Arc<dyn AssemblyRepository>,
}

impl AssemblyService {
    pub fn new(repository: std::sync::Arc<dyn AssemblyRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_assembly(&self, entity_id: String) -> Result<Assembly, String> {
        let assembly = Assembly::new(entity_id);
        self.repository.save(&assembly).await?;
        Ok(assembly)
    }

    pub async fn get_assembly(&self, id: &str) -> Result<Option<Assembly>, String> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_assembly_by_entity(&self, entity_id: &str) -> Result<Option<Assembly>, String> {
        self.repository.find_by_entity(entity_id).await
    }

    pub async fn add_component_to_assembly(
        &self,
        assembly_id: &str,
        component: ComponentInstance,
    ) -> Result<(), String> {
        let mut assembly = self.repository.find_by_id(assembly_id).await?
            .ok_or("Assembly not found".to_string())?;
        
        assembly.add_component(component);
        self.repository.save(&assembly).await
    }

    pub async fn remove_component_from_assembly(
        &self,
        assembly_id: &str,
        component_id: &str,
    ) -> Result<(), String> {
        let mut assembly = self.repository.find_by_id(assembly_id).await?
            .ok_or("Assembly not found".to_string())?;
        
        assembly.remove_component(component_id);
        self.repository.save(&assembly).await
    }
}
