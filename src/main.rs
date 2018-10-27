extern crate actix_web;
extern crate env_logger;
extern crate wp2l2d;

use actix_web::{middleware::Logger, pred, server, App};

use wp2l2d::{config, routes};

fn main() {
    env_logger::init();

    let cfg = config::create();

    let bindhost = format!("{}:{}", cfg.host, cfg.port);

    let apps = move || line_today_app(cfg.clone());

    server::new(apps)
        .bind(&bindhost)
        .unwrap_or_else(|msg| {
            eprintln!("cannot bind to {}: {}", &bindhost, msg);
            std::process::exit(1);
        }).run();
}

fn line_today_app(cfg: config::Config) -> App<config::Config> {
    App::with_state(cfg)
        .middleware(Logger::default())
        .resource("/ping", |r| r.route().filter(pred::Get()).f(routes::ping))
        .resource("/health", |r| {
            r.route().filter(pred::Get()).f(routes::health_report)
        }).resource("/line.xml", |r| {
            r.route().filter(pred::Get()).f(routes::line_xml)
        })
}
