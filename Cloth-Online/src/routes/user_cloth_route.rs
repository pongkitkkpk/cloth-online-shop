use actix_web::web;
use crate::handlers::user_cloth_handler::view_cloth_only_user;


pub fn config(cfg : &mut web::ServiceConfig){
    cfg.service(view_cloth_only_user);

} 