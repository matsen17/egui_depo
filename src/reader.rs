use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Reader {
    pub uuid: Uuid,
    pub name: String,
}

impl Reader {
    pub fn create(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(), 
            name: String::from(name),
        }
    }
}