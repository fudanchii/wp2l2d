use crate::config::Config;
use crate::healthcheck;
use crate::linetoday;

use crate::wordpress::Feed;

use futures::future::Future;

use actix_web::{web::Data, Error, HttpResponse};

pub fn ping() -> &'static str {
    "pong"
}

pub fn health_report(
    cfg: Data<Config>,
) -> impl Future<Item = healthcheck::HealthReport, Error = Error> {
    healthcheck::report(&cfg.wp_feed_url)
}

pub fn line_xml(cfg: Data<Config>) -> impl Future<Item = HttpResponse, Error = Error> {
    Feed::fetch(&cfg.wp_feed_url).and_then(move |feed| linetoday::from(feed, &cfg).build_xml())
}
