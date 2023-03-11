/// This PoC demonstrates the use of rust's actix-web http framework to invoke a 
/// set of thread-unsafe C functions located in a shared library.

use std::collections::HashMap;

use actix_web::{
    middleware,
    rt,
    web,
    App, HttpRequest, HttpServer, HttpResponse, Responder
};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

mod dispatcher;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting http server.");
    run_http_server();
}

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("asdf")
}

async fn no_such_tool(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("no such tool")
}

fn run_http_server() {
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    rt::System::new().block_on(
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(prometheus.clone())
                .service(web::resource("/").route(web::get().to(index)))
                .service(web::resource("/tool").wrap(dispatcher::Dispatcher).to(no_such_tool))
        })
        .bind(("127.0.0.1", 8080)).expect("Failed to bind http server.")
        .workers(1)
        .run(),
    ).expect("Failed to start http server.")
}
