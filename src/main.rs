use actix_web::{
    get,
    web::{Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn index() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

#[get("/-1/error")]
async fn fake_error() -> HttpResponse {
    return HttpResponse::InternalServerError().finish();
}

#[get("/1/{num1}/{num2}")]
async fn logic_test(path: Path<(usize, usize)>) -> HttpResponse {
    let (var1, var2) = path.into_inner();
    let xor_result = var1 | var2;
    let pow_result = usize::pow(xor_result, 3);
    return HttpResponse::Ok().body(format!("{}", pow_result));
}

#[get("/1/{tail:.*}")]
async fn limit_test(req: HttpRequest) -> HttpResponse {
    let split_path: Vec<&str> = req.path().strip_prefix("/1/").unwrap().split("/").collect();

    println!("{:#?}", split_path);
    let mut num: usize = 0;
    for elem in split_path {
        let parsed_elem: usize = elem.parse().unwrap();
        num = num ^ parsed_elem;
        println!("Num: {}", num);
    }
    let pow_result = usize::pow(num, 3);
    return HttpResponse::Ok().body(format!("{}", pow_result));
}

//macros::gen_day_one_routes!();

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index).service(fake_error).service(limit_test);
    };

    Ok(config.into())
}

