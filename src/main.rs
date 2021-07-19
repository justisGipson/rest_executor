use actix_web::{App, HttpServer, web};

extern crate simple_logger;

mod executor {
    pub mod validate_password;
    pub mod execute;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/serv")
                    .service(executor::validate_password::validate_password)
                    .service(executor::execute::execute_command)
            )
    })
        .workers(10)
        .keep_alive(15)
        .bind("127.0.0.1:8080")?
        .run() // method returns an instance of the Server type. Methods of server type could be used for managing the http server
        .await
}
