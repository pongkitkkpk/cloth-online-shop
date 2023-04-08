use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct IdClothCollection{
    pub id_cloth: String,
    pub id_collection: String,
    
}