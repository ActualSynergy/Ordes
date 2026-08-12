// Data access layer for assemblies

use super::domain::Assembly;

#[async_trait::async_trait]
pub trait AssemblyRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Assembly>, String>;
    async fn find_by_entity(&self, entity_id: &str) -> Result<Option<Assembly>, String>;
    async fn save(&self, assembly: &Assembly) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

pub struct InMemoryAssemblyRepository {
    data: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Assembly>>>,
}

impl InMemoryAssemblyRepository {
    pub fn new() -> Self {
        Self {
            data: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl AssemblyRepository for InMemoryAssemblyRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Assembly>, String> {
        Ok(self.data.lock().unwrap().get(id).cloned())
    }

    async fn find_by_entity(&self, entity_id: &str) -> Result<Option<Assembly>, String> {
        Ok(self.data.lock().unwrap()
            .values()
            .find(|a| a.entity_id == entity_id)
            .cloned())
    }

    async fn save(&self, assembly: &Assembly) -> Result<(), String> {
        self.data.lock().unwrap().insert(assembly.id.clone(), assembly.clone());
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        self.data.lock().unwrap().remove(id);
        Ok(())
    }
}
