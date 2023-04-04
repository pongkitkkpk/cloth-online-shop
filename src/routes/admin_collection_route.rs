use actix_web::web;
use crate::handlers::admin_collection_handler::{delete_clothes_collection,add_clothes_collection };

pub fn config(cfg : &mut web::ServiceConfig){
    cfg.service(add_clothes_collection);
    cfg.service(delete_clothes_collection);
   
} 