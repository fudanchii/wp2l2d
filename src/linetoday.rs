use actix_web::{error::ErrorInternalServerError, Error, HttpResponse};
use chrono::{offset::Utc, DateTime};
use config::Config;
use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Error as XMLError, Writer,
};
use rss::{Channel, Item};
use wordpress::Feed;

use std::boxed::Box;

pub struct LineToday(Feed);

enum ChildContent<'a> {
    Text(&'a str),
    Func_C(
        Box<Fn(&mut Writer<Vec<u8>>, &Config, &Channel) -> Result<(), XMLError>>,
        &'a Config,
        &'a Channel,
    ),
    Func_C_I(
        Box<Fn(&mut Writer<Vec<u8>>, &Config, &Channel, &Item) -> Result<(), XMLError>>,
        &'a Config,
        &'a Channel,
        &'a Item,
    ),
}

impl LineToday {
    pub fn build_xml(&self, config: &Config) -> Result<HttpResponse, Error> {
        let xml = self
            .into_xml(config)
            .map_err(|err| ErrorInternalServerError(err))?;

        Ok(HttpResponse::Ok().content_type("application/xml").body(xml))
    }

    fn into_xml(&self, config: &Config) -> Result<Vec<u8>, XMLError> {
        let channel = self.0.borrow_channel();
        let mut writer = Writer::new_with_indent(Vec::new(), ' ' as u8, 2);
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"articles")))?;
        xml_tag(&mut writer, b"UUID", ChildContent::Text(&self.uuid()))?;
        xml_tag(&mut writer, b"time", ChildContent::Text(&self.time()))?;
        for item in channel.items() {
            xml_tag(
                &mut writer,
                b"article",
                ChildContent::Func_C_I(Box::new(article_xml), config, channel, item),
            )?;
        }
        writer.write_event(Event::End(BytesEnd::borrowed(b"articles")))?;
        Ok(writer.into_inner())
    }

    fn uuid(&self) -> String {
        let channel = self.0.borrow_channel();
        let mut uuid = String::from(channel.title());
        uuid.push_str(channel.last_build_date().unwrap_or(""));
        uuid.chars()
            .filter(|&ch| {
                (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9')
            })
            .take(30)
            .collect()
    }

    fn time(&self) -> String {
        let channel = self.0.borrow_channel();
        (DateTime::parse_from_rfc2822(channel.last_build_date().unwrap_or(""))
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now())
            .timestamp() * 1000)
            .to_string()
    }
}

pub fn from(feed: Feed) -> LineToday {
    LineToday(feed)
}

fn xml_tag(
    writer: &mut Writer<Vec<u8>>,
    tag_name: &[u8],
    content: ChildContent,
) -> Result<(), XMLError> {
    writer.write_event(Event::Start(BytesStart::borrowed_name(tag_name)))?;
    match content {
        ChildContent::Text(txt) => {
            writer.write_event(Event::Text(BytesText::from_plain_str(txt)))?;
        }
        ChildContent::Func_C(f, cfg, c) => (*f)(writer, cfg, c)?,
        ChildContent::Func_C_I(f, cfg, c, i) => (*f)(writer, cfg, c, i)?,
    }
    writer.write_event(Event::End(BytesEnd::borrowed(tag_name)))?;
    Ok(())
}

fn article_xml(
    writer: &mut Writer<Vec<u8>>,
    config: &Config,
    channel: &Channel,
    item: &Item,
) -> Result<(), XMLError> {
    Ok(())
}
