use futures::future::Future;
use serde_derive::Serialize;
use std::time;

use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, Responder};

#[derive(Serialize, Debug)]
pub struct HealthReport {
    url: String,
    response_code: u16,
    response_type: String,
    response_time: u64,
    health: bool,
}

impl Responder for HealthReport {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        Ok(HttpResponse::Ok().json(self))
    }
}

impl HealthReport {
    fn set_response(&mut self, status: u16, content_type: &str) {
        self.response_code = status;
        self.response_type = content_type.to_string();
        self.health = (self.response_code / 100) < 4;
    }

    fn set_time_since(&mut self, previous_time: time::Instant) {
        let duration = time::Instant::now().duration_since(previous_time);

        self.response_time = (duration.as_secs() * 1000) + duration.subsec_millis() as u64;
    }
}

pub fn report(remote_url: &str) -> impl Future<Item = HealthReport, Error = Error> {
    let mut health_report = HealthReport {
        url: remote_url.to_string(),
        response_code: 0,
        response_type: "".to_string(),
        response_time: 0,
        health: false,
    };

    let before_request = time::Instant::now();

    awc::Client::default()
        .head(remote_url)
        .send()
        .map_err(Error::from)
        .and_then(move |response| {
            health_report.set_response(response.status().as_u16(), response.content_type());
            health_report.set_time_since(before_request);
            Ok(health_report)
        })
}
