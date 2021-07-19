// this function will get a JSON request with the username and the password for a user and will return a boolean result
// this and validate_password will be sharing a root path as serv

use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::io::{self, Write};

#[derive(Deserialize)]
pub struct Request{
  commands: String,
}

#[derive(Serialize)]
pub struct Response {
  result: bool,
}

#[post("/execute")]
pub async fn execute_command(request: web::Json<Request>) -> impl Responder {
  info!("validating password for {}", request.commands);

  let process = Command::new("sh")
    .arg(&request.commands)
    .status()
    .expect("Failed to execute command");

  info!{"status: {}", &process.to_string()};

  if process.success() {
    HttpResponse::Ok().json(Response { result: true })
  } else {
    HttpResponse::Ok().json(Response { result: false })
  }
}
