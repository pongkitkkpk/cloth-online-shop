use actix_web::web;
use crate::handlers::user_collection_handler::{view_cloth_all_user,view_collection_only_user};


pub fn config(cfg : &mut web::ServiceConfig){
    cfg.service(view_cloth_all_user);
    cfg.service(view_collection_only_user);


} 