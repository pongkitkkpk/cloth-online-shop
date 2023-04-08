use actix_web::{delete, post, web,  HttpResponse,  Responder};

use crate::models::mcollectioncloth::{AllDetailClothCollection, BodyClothCollection};

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;


//***************** Admin สามารถเพิ่ม Clothes's Collection ได้ ********************************//

#[post("/admin/collection")]
async fn add_clothes_collection(input_form: web::Json<AllDetailClothCollection>) -> impl Responder {
    info!("admin add (http:post)");

    let new_collection = input_form.into_inner();


    let name_collection_for_add = "";

    let mut _data = AllDetailClothCollection {
        id_collection: new_collection.id_collection.to_string(),
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


    //รวมกับ message ของระบบเพื่อแจ้งเตือนการแสดงผล

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_about_all: AllDetailClothCollection,
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
struct EventidCollection {
    id_collection: String,
}

//****************** Admin สามารถลบ Clothes's Collection ได้ **********************//

#[delete("/admin/collection/{id_collection}")]
async fn delete_clothes_collection(
    id: web::Path<String>,
    input_form: web::Json<EventidCollection>,
) -> impl Responder {
    info!("Delete Clothes's By IDCollection ");

    let id: String = id.to_string();
    let collection_data = input_form.into_inner();
    // let id:String = id.to_string();
    //สมมติข้อมูลที่ต้องการลบ
    let mut old_collection = BodyClothCollection {
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
        old_collection = BodyClothCollection {
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
        detail_about_all: BodyClothCollection,
        message: String,
    }

    let web_response = WebResponse {
        detail_about_all: old_collection,
        message: message_delete.to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}




