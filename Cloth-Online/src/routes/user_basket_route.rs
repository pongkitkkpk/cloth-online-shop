use actix_web::web;
use crate::handlers::user_basket_handler::{view_order_basket,add_order2basket,delete_clothformbasket,confirm_order};


pub fn config(cfg : &mut web::ServiceConfig){
    cfg.service(view_order_basket);
    cfg.service(add_order2basket);
    cfg.service(delete_clothformbasket);
    cfg.service(confirm_order);

} 