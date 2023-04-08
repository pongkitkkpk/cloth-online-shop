use actix_web::{post,web, HttpResponse,Responder};
use log::{info};
use serde_json::json;
use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize)]
pub struct ClothEditList{
    pub id_cloth: String,
    pub description_cloth: String,
    pub cost_cloth: i32,
    pub date : String
}

//********************** Admin สามารถแก้ไขคำอธิบายและราคาของ Clothes's Collection ได้พร้อมลงวันที่การแก้ไขข้อมูลครั้งล่าสุด *****************//
#[post("/admin/edit/{id_cloth}")]
async fn edit_clothes_collection(id:web::Path<String>,input_form:web::Json<ClothEditList>) -> impl Responder {
    info!("Edit Clothes's By id cloth");

    let id:String = id.to_string();
    let id_edit = input_form.into_inner();

    let mut _edit_detail1 = ClothEditList{
            id_cloth: id_edit.id_cloth.to_string(),
            description_cloth : id_edit.description_cloth.to_string(),
            cost_cloth : id_edit.cost_cloth,
            date : id_edit.date.to_string(),
        };
    let mut message_edit ="Your new correction are complete! Please check your detail about your new correction.";
    //เช็คว่าurlกับbodyตรงกันไหม
    if id.to_string() != id_edit.id_cloth.to_string(){
        message_edit = "ID not found or Not Match !!";
        _edit_detail1 = ClothEditList{
            id_cloth :"No Data".to_string(),
            description_cloth : "No Data".to_string(),
            cost_cloth : 0,
            date : "No Data".to_string(),
        };
    }else{
        message_edit = "Edit Clothes's Collection Complete!";
        
    }

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        detail_edit: ClothEditList,
        message: String
    }

    let web_response = WebResponse {
        detail_edit: _edit_detail1,
        message:message_edit.to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}



