use wordpress::Feed;
use actix_web::{HttpResponse, Error};
use quick_xml::Writer;
use std::io::Cursor;
use config::Config;

pub struct LineToday(Feed);

impl LineToday {
    pub fn build_xml(&self, config: &Config) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok()
            .content_type("application/xml")
            .body("<yay></yay>"))
    }
}

pub fn from(feed: Feed) -> LineToday {
    LineToday(feed)
}
