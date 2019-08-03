use actix_web::{middleware::Logger, web, App, HttpServer};

use wp2l2d::{config, routes};

fn main() {
    env_logger::init();

    let cfg = config::create();

    let bindhost = format!("{}:{}", cfg.host, cfg.port);

    HttpServer::new(move || {
        App::new()
            .data(cfg.clone())
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind(&bindhost)
    .unwrap_or_else(|msg| {
        eprintln!("cannot bind to {}: {}", &bindhost, msg);
        std::process::exit(1);
    })
    .run()
    .unwrap_or_else(|_| {
        eprintln!("cannot run server.");
        std::process::exit(1);
    });
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("ping", web::get().to(routes::ping))
            .route("health", web::get().to_async(routes::health_report))
            .route("line.xml", web::get().to_async(routes::line_xml)),
    );
}
