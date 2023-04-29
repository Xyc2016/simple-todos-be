use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use crate::{actions, common_types::AppState};

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.db_conn_pool.get().expect("Failed to get db connection from pool.");
    let action = actions::todos::TodoAction::new(&mut conn);
    "a"
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        index
    );
}

