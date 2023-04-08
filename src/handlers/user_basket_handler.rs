use crate::models::mbasket::IdClothCollection;
use crate::models::mcollectioncloth::AllDetailClothCollection;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::{info};
use serde::{Deserialize, Serialize};
use serde_json::json;


//**********  User สามารถดูรายการสั่งซื้อสินค้าในตะกร้าได้ ************//

#[get("/user/basket")]
async fn view_order_basket() -> impl Responder {
    info!("Show item in basket");

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
        description_cloth: "...Description...s".to_string(),
        cost_cloth: 699,
        type_cloth: "Bottoms".to_string(),
        material_of_cloth: "Cotton".to_string(),
        sex_cloth: "Male".to_string(),
        stock_of_cloth: 14,
        date: "2023-03-21".to_string(),
    };

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        id_basket: String,
        detail_all_1: AllDetailClothCollection,
        detail_all_2: AllDetailClothCollection,
        detail_all_3: AllDetailClothCollection,
        message: String,
    }

    let web_response = WebResponse {
        id_basket: "basket0001".to_string(),
        detail_all_1: _data1,
        detail_all_2: _data2,
        detail_all_3: _data3,
        message: "That cloth in your basket.".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}

// //************* User เพิ่มเสื้อผ้าที่ต้องการซื้อลงในตะกร้า *********************//

#[post("/user/basket/{id_basket}")]
async fn add_order2basket(
    id: web::Path<String>,
    input_form: web::Json<IdClothCollection>,
) -> impl Responder {
    info!("user add Cloth to basket. (http:post)");

    let mut _iddata = IdClothCollection {
        id_collection: input_form.id_collection.to_string(),
        id_cloth: input_form.id_cloth.to_string(),
    };

    //สมมติ(Test)[/admin/addition/Test01]

    //รวมกับ message ของระบบเพื่อแจ้งเตือนการแสดงผล

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        id_basket: String,
        detail_id: IdClothCollection,
        message: String,
    }

    let web_response = WebResponse {
        id_basket: id.to_string(),
        detail_id: _iddata,
        message: "Add Cloth To Basket Complete!".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Created().json(response)
}




#[derive(Serialize, Deserialize)]
    struct Idfordelete {
        id_basket: String,
        id_cloth:String
    }

//*********** User ลบเสื้อผ้าที่ไม่ต้องการซื้อออกจากตะกร้า *************//

#[delete("/user/basket/{id_basket}/{id_cloth}")]
async fn delete_clothformbasket(
    id: web::Path<Idfordelete>,
) -> impl Responder {
    info!("user delete cloth form basket. (http:delete)");

    let mut _id_delete = Idfordelete{
        id_cloth : id.id_cloth.to_string(),
        id_basket : id.id_basket.to_string(),
    };

    //สมมติ(Test)[/admin/addition/Test01]

    //รวมกับ message ของระบบเพื่อแจ้งเตือนการแสดงผล

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        id: Idfordelete,
        message: String,
    }

    let web_response = WebResponse {
        id: _id_delete,
        message: "Delete cloth in basket are complete !".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}




#[derive(Serialize, Deserialize)]
struct Confirmid {
    id_basket: String,
}

//***************** User ตกลงสั่งซื้อสินค้าในตะกร้า S********************//

#[post("/user/basket")]
async fn confirm_order(input_form:web::Json<Confirmid>) -> impl Responder {
    info!("confirm Clothes's By IDCollection ");

    let id_basket = Confirmid{
        id_basket : input_form.id_basket.to_string()
    };
    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        id_basket: Confirmid,
        message: String,
    }

    let web_response = WebResponse {
        id_basket: id_basket,
        message:"Your order is done! Thank you. Have a nice day.".to_string(),
    };
    let response = json!(web_response);

    HttpResponse::Ok().json(response)

}