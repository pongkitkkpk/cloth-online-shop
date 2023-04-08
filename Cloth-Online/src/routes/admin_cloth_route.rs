use actix_web::web;
use crate::handlers::admin_cloth_handler::edit_clothes_collection;

pub fn config(cfg : &mut web::ServiceConfig){
    cfg.service(edit_clothes_collection);
   
} 