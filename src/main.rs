use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn index(_: HttpRequest) -> impl Responder {
    let body = oxicsv::get_json_records();

    HttpResponse::Ok().json(body)
}

fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:12345")
        .expect("Can not bind to port 12345")
        .run()
        .unwrap();
}
