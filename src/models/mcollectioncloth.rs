

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct All_Detail_List {
    pub name_cloth: String,
    pub id_cloth: String,
    pub name_collection: String,
    pub id_collection: String,
    pub description_cloth: String,
    pub cost_cloth: i32,
    pub type_cloth: String,
    pub material_of_cloth: String,
    pub sex_cloth: String,
    pub stock_of_cloth: i32,
    pub date : String
}

#[derive(Serialize,Deserialize)]
pub struct Collection_Cloth_ID_List{
    pub name_cloth: String,
    pub id_cloth: String,
    pub name_collection: String,
    pub id_collection: String,
}

#[derive(Serialize,Deserialize)]
pub struct Collection_Detail_List{
    pub description_cloth: String,
    pub cost_cloth: i32,
    pub type_cloth: String,
    pub material_of_cloth: String,
    pub sex_cloth: String,
    pub stock_of_cloth: i32,
    pub date : String
}



