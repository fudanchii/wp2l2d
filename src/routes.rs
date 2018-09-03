use config;
use healthcheck;
use linetoday;
use wordpress::Feed;

use actix_web::{HttpRequest, Responder};

pub fn ping(_req: &HttpRequest<config::Config>) -> &'static str {
    "pong"
}

pub fn health_report(req: &HttpRequest<config::Config>) -> impl Responder {
    healthcheck::report(&req.state().wp_feed_url)
}

pub fn line_xml(req: &HttpRequest<config::Config>) -> impl Responder {
    let url = &req.state().wp_feed_url;
    linetoday::from(Feed::fetch(url)?).build_xml(req.state())
}
