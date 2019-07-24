use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn index(_: HttpRequest) -> impl Responder {
    let body = oxicsv::get_json_records();

    HttpResponse::Ok().json(body)
}

fn get(idx: web::Path<usize>) -> impl Responder {
    let index = idx.into_inner();
    match oxicsv::get_records().get(index) {
        Some(r) => HttpResponse::Ok().json(r.to_json()),
        None => HttpResponse::NotFound().body(format!("record {} not found", index)),
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get/{idx}", web::get().to(get))
    })
    .bind("127.0.0.1:12345")
    .expect("Can not bind to port 12345")
    .run()
    .unwrap();
}
