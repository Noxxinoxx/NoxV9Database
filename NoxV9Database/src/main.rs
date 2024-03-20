use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder}; 
mod database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let test = database::testFunction("or not hej");

    println!("hej {}", test);

    HttpServer::new(|| {
        App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello))
    }).bind(("localhost", 8080))?.run().await
    
}
