use crate::models::mcollectioncloth::AllDetailClothCollection;
use actix_web::{get, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;


// ******************* Admin สามารถดูหน้า Home page ได้ในมุมมองเดียวกันกับที่ User เห็นตอนเลือก S*********************//
#[get("/admin/homepage")]
async fn show_detail_admin() -> impl Responder {
    info!("Show Homepage Like User's view For Admin");

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
        message: "This is Homepage when user use this web to buy your clothes.".to_string(),
    };

    let response = json!(web_response);

    HttpResponse::Ok().json(response)
}
