use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct idClothCollection{
    pub id_cloth: String,
    pub id_collection: String,
    
}