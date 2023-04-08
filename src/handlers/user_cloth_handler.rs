use crate::models::mcollectioncloth::AllDetailClothCollection;
use actix_web::{get, web, HttpResponse, Responder};
use log::{info};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct EventidCloth {
    id_cloth: String,
}

//************ User ดูแค่ Cloth(เสื้อผ้า) ที่สนใจซื้อเพียง 1 ตัวเท่านั้น **********//

#[get("/user/cloth/{id_cloth}")]
async fn view_cloth_only_user(id: web::Path<String>,input_form: web::Json<EventidCloth>) -> impl Responder {
    info!("Show Only Cloth");

    let id: String = id.to_string();
    let cloth_id = input_form.into_inner();
    // let id:String = id.to_string();
    let mut _data1 = AllDetailClothCollection {
        name_cloth: "Snowman Overcoat".to_string(),
        id_cloth: cloth_id.id_cloth.to_string(),
        name_collection: "Be cool in Winter".to_string(),
        id_collection: "Cool_win".to_string(),
        description_cloth: "...Description...".to_string(),
        cost_cloth: 1199,
        type_cloth: "Tops".to_string(),
        material_of_cloth: "Wool".to_string(),
        sex_cloth: "Female".to_string(),
        stock_of_cloth: 42,
        date: "2023-03-20".to_string(),
    };
    let mut message = "Show Only Cloth user want to buy";
    //เช็คว่าid_Collection ว่าurl ตรงกับ bodyไหม
    if id.to_string() != cloth_id.id_cloth.to_string() {
        _data1 = AllDetailClothCollection {
            name_cloth: "No Data".to_string(),
            id_cloth: "No Data".to_string(),
            name_collection: "No Data".to_string(),
            id_collection: "No Data".to_string(),
            description_cloth: "No Data".to_string(),
            cost_cloth: 0,
            type_cloth: "No Data".to_string(),
            material_of_cloth: "No Data".to_string(),
            sex_cloth: "No Data".to_string(),
            stock_of_cloth: 0,
            date: "No Data".to_string(),
        };
        message = "ID not found or Not Match !!";
    } else {
        message = "Show Only Clothes's Detail Complete!";
    }

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_cloth: AllDetailClothCollection,
        message: String,
    }

    let web_response = WebResponse {
        detail_cloth: _data1,
        message: message.to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}


