// use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}\n", &name)
// }

