use actix_web::{
    client,
    client::ClientResponse,
    error::{ErrorInternalServerError, ErrorPreconditionFailed},
    Error, HttpMessage,
};
use futures::Future;
use rss::Channel;

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

        let feed = response.body().wait()?;

        let channel =
            Channel::read_from(feed.as_ref()).map_err(|err| ErrorInternalServerError(err))?;

        Ok(Feed(channel))
    }

    pub fn channel(&self) -> Channel {
        self.0.clone()
    }
}

fn assert_valid_xml(resp: &ClientResponse) -> Result<(), Error> {
    if resp.status() == 200 && resp.content_type() == "application/rss+xml" {
        return Ok(());
    }
    Err(ErrorPreconditionFailed(format!(
        "unexpected response code {} and type {} from backend",
        resp.status(),
        resp.content_type()
    )))
}
