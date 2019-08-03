use actix_web::{
    error::{ErrorInternalServerError, ErrorPreconditionFailed},
    Error, HttpMessage,
};
use futures::future::Future;
use rss::Channel;

pub struct Feed(Channel);

impl Feed {
    pub fn fetch(url: &str) -> impl Future<Item = Self, Error = Error> {
        awc::Client::default()
            .get(url)
            .send()
            .map_err(Error::from)
            .and_then(|mut response| {
                response.body().map_err(Error::from).and_then(move |feed| {
                    assert_valid_xml(response.status().as_u16(), response.content_type())?;

                    let channel = Channel::read_from(feed.as_ref())
                        .map_err(|err| ErrorInternalServerError(err))?;

                    Ok(Feed(channel))
                })
            })
    }

    pub fn borrow_channel(&self) -> &Channel {
        &self.0
    }
}

fn assert_valid_xml(status: u16, content_type: &str) -> Result<(), Error> {
    if status == 200 && content_type == "application/rss+xml" {
        return Ok(());
    }
    Err(ErrorPreconditionFailed(format!(
        "unexpected response code {} and type {} from backend",
        status, content_type
    )))
}
