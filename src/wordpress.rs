use actix_web::{client, client::ClientResponse, HttpMessage, Error};
use rss::Channel;
use futures::Future;
use std::str::FromStr;

pub struct Feed(Channel);

impl Feed {
    pub fn fetch(url: &str) -> Result<Self, Error> {
        let response = client::get(url)
            .finish()
            .unwrap()
            .send()
            .map_err(Error::from)
            .wait()?;

        assert_valid_xml(&response)?;

        let feed = response.body()
            .from_err()
            .wait()?;

        let channel = Channel::read_from(feed.as_ref())?;

        Ok(Feed(channel))
    }

    pub fn channel(&self) -> Channel {
        self.0
    }
}

fn assert_valid_xml(resp: &ClientResponse) -> Result<(), Error> {
    if resp.status() == 200 && resp.content_type() == "application/rss+xml" {
        return Ok(());
    }

    Error(format!("response error ({}), response type: {}", resp.status(), resp.content_type()))
}
