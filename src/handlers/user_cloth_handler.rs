use crate::models::mcollectioncloth::All_Detail_List;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Event_id_cloth {
    id_cloth: String,
}

#[get("/user/cloth/{id_cloth}")]
async fn view_cloth_only_user(id: web::Path<String>,input_form: web::Json<Event_id_cloth>,) -> impl Responder {
    info!("Show Homepage Like User's view");

    let id: String = id.to_string();
    let cloth_id = input_form.into_inner();
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
    let mut message = "";
    //เช็คว่าid_Collection ว่าurl ตรงกับ bodyไหม
    if id.to_string() != cloth_id.id_cloth.to_string() {
        _data1 = All_Detail_List {
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
        detail_cloth: All_Detail_List,
        message: String,
    }

    let web_response = WebResponse {
        detail_cloth: _data1,
        message: message.to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(view_cloth_only_user))
        .bind("127.0.0.1:8082")?
        .run()
        .await
}
