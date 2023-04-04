use crate::models::mcollectioncloth::All_Detail_List;
use actix_web::{get, web, HttpResponse, Responder,App,HttpServer};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[get("/user/collection")]
async fn view_cloth_all_user() -> impl Responder {
    info!("Show Homepage Like User's view");

    let mut _data1 = All_Detail_List {
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
    let mut _data2 = All_Detail_List {
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
    let mut _data3 = All_Detail_List {
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
        detail_all_1: All_Detail_List,
        detail_all_2: All_Detail_List,
        detail_all_3: All_Detail_List,
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
struct Event_id_collection {
    id_collection: String,
}

#[get("/user/collection/{id_collection}")]
async fn view_collection_only_user(
    id: web::Path<String>,
    input_form: web::Json<Event_id_collection>,
) -> impl Responder {
    info!("Show Only Collection Like User's view");

    let id: String = id.to_string();
    let collection_id = input_form.into_inner();
    // let id:String = id.to_string();
    let mut _data1 = All_Detail_List {
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
    let mut _data2 = All_Detail_List {
        name_cloth: "Snowman Overcoat".to_string(),
        id_cloth: "Cool_win_2".to_string(),
        name_collection: "Wind Winter".to_string(),
        id_collection: "Cool_win".to_string(),
        description_cloth: "...Description...".to_string(),
        cost_cloth: 1099,
        type_cloth: "Tops".to_string(),
        material_of_cloth: "Cotton".to_string(),
        sex_cloth: "male".to_string(),
        stock_of_cloth: 87,
        date: "2023-03-18".to_string(),
    };
    let mut message = "";
    //เช็คว่าid_Collection ว่าurl ตรงกับ bodyไหม
    if id.to_string() != collection_id.id_collection.to_string() {
        message = "ID not found or Not Match !!";
    } else {
        message = "Show Only Clothes's Collection Complete!";
    }

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_all_1: All_Detail_List,
        detail_all_2: All_Detail_List,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(view_collection_only_user))
        .bind("127.0.0.1:8082")?
        .run()
        .await
}
