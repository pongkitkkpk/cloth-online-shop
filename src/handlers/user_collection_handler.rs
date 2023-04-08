use crate::models::mcollectioncloth::AllDetailClothCollection;
use actix_web::{get, web,  HttpResponse,  Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

//*********** User ดู Cothes's Collection ทั้งหมดในหน้า Homepage ********//

#[get("/user/collection")]
async fn view_cloth_all_user() -> impl Responder {
    info!("Show Homepage Like User's view");

    let mut _data1 = AllDetailClothCollection {
        name_cloth: "Snowman Overcoat".to_string(),
        id_cloth: "Cool_win_1".to_string(),
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
    let mut _data2 = AllDetailClothCollection {
        name_cloth: "White Miniskirt".to_string(),
        id_cloth: "Y2K_1".to_string(),
        name_collection: "Back to Y2K".to_string(),
        id_collection: "Y2K".to_string(),
        description_cloth: "...Description...".to_string(),
        cost_cloth: 559,
        type_cloth: "Bottoms".to_string(),
        material_of_cloth: "Yeans".to_string(),
        sex_cloth: "Female".to_string(),
        stock_of_cloth: 20,
        date: "2023-03-20".to_string(),
    };
    let mut _data3 = AllDetailClothCollection {
        name_cloth: "Cargo Pant".to_string(),
        id_cloth: "Into_wood_1".to_string(),
        name_collection: "Into the wood".to_string(),
        id_collection: "Into_wood".to_string(),
        description_cloth: "...Description...".to_string(),
        cost_cloth: 699,
        type_cloth: "Bottoms".to_string(),
        material_of_cloth: "Cotton".to_string(),
        sex_cloth: "Male".to_string(),
        stock_of_cloth: 14,
        date: "2023-03-21".to_string(),
    };

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_all_1: AllDetailClothCollection,
        detail_all_2: AllDetailClothCollection,
        detail_all_3: AllDetailClothCollection,
        message: String,
    }

    let web_response = WebResponse {
        detail_all_1: _data1,
        detail_all_2: _data2,
        detail_all_3: _data3,
        message: "This is Homepage for user buy clothes.".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}


#[derive(Serialize, Deserialize)]
struct EventidCollection {
    id_collection: String,
}


//**************** User ดูแค่ Clothes's Collection ที่สนใจซื้อเพียง 1 Collection เท่านั้น ******************//

#[get("/user/collection/{id_collection}")]
async fn view_collection_only_user(
    id: web::Path<String>,
    input_form: web::Json<EventidCollection>,
) -> impl Responder {
    info!("Show Only Collection");

    let id: String = id.to_string();
    let collection_id = input_form.into_inner();
    // let id:String = id.to_string();
    let mut _data1 = AllDetailClothCollection {
        name_cloth: "Snowman Overcoat".to_string(),
        id_cloth: "Cool_win_1".to_string(),
        name_collection: "Be cool in Winter".to_string(),
        id_collection: collection_id.id_collection.to_string(),
        description_cloth: "...Description...".to_string(),
        cost_cloth: 1199,
        type_cloth: "Tops".to_string(),
        material_of_cloth: "Wool".to_string(),
        sex_cloth: "Female".to_string(),
        stock_of_cloth: 42,
        date: "2023-03-20".to_string(),
    };
    let mut _data2 = AllDetailClothCollection {
        name_cloth: "Snowman Overcoat".to_string(),
        id_cloth: "Cool_win_2".to_string(),
        name_collection: "Wind Winter".to_string(),
        id_collection: collection_id.id_collection.to_string(),
        description_cloth: "...Description...".to_string(),
        cost_cloth: 1099,
        type_cloth: "Tops".to_string(),
        material_of_cloth: "Cotton".to_string(),
        sex_cloth: "male".to_string(),
        stock_of_cloth: 87,
        date: "2023-03-18".to_string(),
    };
    let mut message = "Show Only Collection";
    //เช็คว่าid_Collection ว่าurl ตรงกับ bodyไหม
    if id.to_string() != collection_id.id_collection.to_string() {
        message = "ID not found or Not Match !!";
        _data1 = AllDetailClothCollection {
            name_cloth: "No Data".to_string(),
            id_cloth: "No Data".to_string(),
            name_collection: "No Data".to_string(),
            id_collection: "No Data".to_string(),
            description_cloth: "...No Data...".to_string(),
            cost_cloth: 0,
            type_cloth: "No Data".to_string(),
            material_of_cloth: "No Data".to_string(),
            sex_cloth: "No Data".to_string(),
            stock_of_cloth: 0,
            date: "No Data".to_string(),
        };
        _data2 = AllDetailClothCollection {
            name_cloth: "No Data".to_string(),
            id_cloth: "No Data".to_string(),
            name_collection: "No Data".to_string(),
            id_collection: "No Data".to_string(),
            description_cloth: "...No Data...".to_string(),
            cost_cloth: 0,
            type_cloth: "No Data".to_string(),
            material_of_cloth: "No Data".to_string(),
            sex_cloth: "No Data".to_string(),
            stock_of_cloth: 0,
            date: "No Data".to_string(),
        };
    } else {
        message = "Show Only Clothes's Collection Complete!";
    }
    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_all_1: AllDetailClothCollection,
        detail_all_2: AllDetailClothCollection,
        message: String,
    }

    let web_response = WebResponse {
        detail_all_1: _data1,
        detail_all_2: _data2,
        message: message.to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}


