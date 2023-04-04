use actix_web::web;
use crate::handlers::admin_homepage_handler::show_detail_admin;


pub fn config(cfg : &mut web::ServiceConfig){
    cfg.service(show_detail_admin);

} 