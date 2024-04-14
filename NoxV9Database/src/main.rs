use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder}; 
mod database;
mod init;
mod hashing;
mod databasewriter;


/*

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
    let mut options : Vec<String> = Vec::new();
    options.push("V9".to_string()); 
    options.push("V8".to_string());

    init::init(options);

    HttpServer::new(|| {
        App::new().service(hello).service(echo).route("/hey", web::get().to(manual_hello))
    }).bind(("localhost", 8080))?.run().await
    
}

*/

fn main() {
    let user : database::User = database::User::new();
    user.set_user_object("Noa".to_string(), "Noxdator@outlook.com".to_string(), "MyPasswordIsStrong!".to_string());
}
