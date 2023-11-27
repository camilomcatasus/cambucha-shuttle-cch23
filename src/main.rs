use actix_web::{get, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn index() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

#[get("/-1/error")]
async fn fake_error() -> HttpResponse {
    return HttpResponse::InternalServerError().finish();
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index).service(fake_error);
    };

    Ok(config.into())
}
