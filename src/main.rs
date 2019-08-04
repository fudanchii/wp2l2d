use actix_web::{middleware::Logger, web, App, HttpServer};

use wp2l2d::{config, routes};

fn main() {
    env_logger::init();

    serve().unwrap_or_else(|err| {
        eprintln!("cannot start server: {:?}", err);
        std::process::exit(1)
    });
}

fn serve() -> Result<(), std::io::Error> {
    let cfg = config::create();
    let bindhost = format!("{}:{}", cfg.host, cfg.port);

    HttpServer::new(move || {
        App::new()
            .data(cfg.clone())
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind(&bindhost)?
    .run()
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("ping", web::get().to(routes::ping))
            .route("health", web::get().to_async(routes::health_report))
            .route("line.xml", web::get().to_async(routes::line_xml)),
    );
}
