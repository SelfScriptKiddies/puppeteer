use actix_web::{App, HttpServer};
mod routes;

struct WebApplicationData {

}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {


    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}