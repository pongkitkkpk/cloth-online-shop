use std::ptr::eq;

use actix_web::{delete, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::mcollectioncloth::All_Detail_List;
use crate::models::mcollectioncloth::*;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[post("/admin/collection")]
async fn add_clothes_collection(input_form: web::Json<All_Detail_List>) -> impl Responder {
    info!("admin add (http:post)");

    let new_collection = input_form.into_inner();

    let id_collection_for_add = "S_2023";

    let mut name_collection_for_add = "";

    let mut _data = All_Detail_List {
        id_collection: id_collection_for_add.to_string(),
        name_collection: new_collection.name_collection.to_string(),
        id_cloth: new_collection.id_cloth.to_string(),
        name_cloth: new_collection.name_cloth.to_string(),
        description_cloth: new_collection.description_cloth.to_string(),
        cost_cloth: new_collection.cost_cloth,
        type_cloth: new_collection.type_cloth.to_string(),
        material_of_cloth: new_collection.material_of_cloth.to_string(),
        sex_cloth: new_collection.sex_cloth.to_string(),
        stock_of_cloth: new_collection.stock_of_cloth,
        date: new_collection.date.to_string(),
    };

    //สมมติ(Test)[/admin/addition/Test01]
    if new_collection.id_collection == "S_2023" {
        name_collection_for_add = "Collecton name for test";
    }

    //รวมกับ message ของระบบเพื่อแจ้งเตือนการแสดงผล

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_about_all: All_Detail_List,
        message: String,
    }

    let web_response = WebResponse {
        detail_about_all: _data,
        message: "Add Clothes's Collection Complete!".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Created().json(response)
}

#[derive(Serialize, Deserialize)]
struct Event_id_collection {
    id_collection: String,
}
//ของจริง
#[delete("/admin/collection/{id_collection}")]
async fn delete_clothes_collection(
    id: web::Path<String>,
    input_form: web::Json<Event_id_collection>,
) -> impl Responder {
    info!("Delete Clothes's By IDCollection ");

    let id: String = id.to_string();
    let collection_data = input_form.into_inner();
    // let id:String = id.to_string();
    //สมมติข้อมูลที่ต้องการลบ
    let mut old_collection = Collection_Detail_List {
        description_cloth: "This is item want for remove".to_string(),
        cost_cloth: 585,
        type_cloth: "Tops".to_string(),
        material_of_cloth: "Lien".to_string(),
        sex_cloth: "Male".to_string(),
        stock_of_cloth: 54,
        date: "2023-12-5".to_string(),
    };
    let mut message_delete = "";
    //เช็คว่าurlกับbodyตรงกันไหม
    if id.to_string() != collection_data.id_collection.to_string() {
        message_delete = "ID not found or Not Match !!";
        old_collection = Collection_Detail_List {
            description_cloth: "No Data".to_string(),
            cost_cloth: 0,
            type_cloth: "No Data".to_string(),
            material_of_cloth: "No Data".to_string(),
            sex_cloth: "No Data".to_string(),
            stock_of_cloth: 0,
            date: "No Data".to_string(),
        };
    } else {
        message_delete = "Delete Clothes's Collection Complete!";
    }

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_about_all: Collection_Detail_List,
        message: String,
    }

    let web_response = WebResponse {
        detail_about_all: old_collection,
        message: message_delete.to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(delete_clothes_collection))
        .bind("127.0.0.1:8082")?
        .run()
        .await
}

// #[derive(Serialize, Deserialize)]
// struct detaillist_New {
//     name_cloth: String,
//     id_cloth: String,
//     name_collection: String,
//     id_collection: String,
//     description_cloth: String,
//     cost_cloth: i32,
//     type_cloth: String,
//     material_of_cloth: String,
//     sex_cloth: String,
//     stock_of_cloth: i32,
//     date: String,
// }

//ใช้สำหรับ admin เพิ่มข้อมูลเสื้อผ้า
