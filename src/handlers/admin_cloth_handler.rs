use actix_web::{post,App,web, HttpResponse,Responder,HttpServer};
use log::{debug, info};
use serde_json::json;
use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize)]
pub struct Collection_Edit_List{
    pub id_collection: String,
    pub description_cloth: String,
    pub cost_cloth: i32,
    pub date : String
}



//ของจริง
#[post("/admin/edit/{id_collection}")]
async fn edit_clothes_collection(id:web::Path<String>,input_form:web::Json<Collection_Edit_List>) -> impl Responder {
    info!("Edit Clothes's By IDCollection ");

    let id:String = id.to_string();
    let id_edit = input_form.into_inner();

    let mut _edit_detail1 = Collection_Edit_List{
            id_collection: id_edit.id_collection.to_string(),
            description_cloth : id_edit.description_cloth.to_string(),
            cost_cloth : id_edit.cost_cloth,
            date : id_edit.date.to_string(),
        };
    let mut message_edit ="";
    //เช็คว่าurlกับbodyตรงกันไหม
    if id.to_string() != id_edit.id_collection.to_string(){
        message_edit = "ID not found or Not Match !!";
        _edit_detail1 = Collection_Edit_List{
            id_collection :"No Data".to_string(),
            description_cloth : "No Data".to_string(),
            cost_cloth : 0,
            date : "No Data".to_string(),
        };
    }else{
        message_edit = "Edit Clothes's Collection Complete!";
        
    }

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_edit: Collection_Edit_List,
        message: String
    }

    let web_response = WebResponse {
        detail_edit: _edit_detail1,
        message:"Edit Comp".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(edit_clothes_collection)
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}

