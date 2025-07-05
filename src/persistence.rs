use crate::store::KeyValueStore;
use tokio::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use anyhow;

#[derive(Serialize, Deserialize)]
struct PersistentData {
    data: Vec<(String, String)>
}

impl KeyValueStore {
    pub async fn load(&self, path:&str) -> anyhow::Result<()> {
       if Path::new(path).exists() {
           let content = fs::read_to_string(path).await?;
           let persisted_data: PersistentData = serde_json::from_str(&content)?;
           
           {
                let mut map = self.map.lock().await;
                map.extend(persisted_data.data);
           }
       }
       Ok(()) 
    }

    pub async fn save(&self, path:&str) -> anyhow::Result<()> {
        let data = {
            let map = self.map.lock().await;
            PersistentData {
                data: map.iter().map(|(key, value)|(key.clone(), value.clone())).collect()
            }
        };

        let content = serde_json::to_string(&data)?;
        fs::write(path, content).await?;
        Ok(())
    }

}