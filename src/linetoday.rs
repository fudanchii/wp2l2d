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

pub struct LineToday<'a> {
    feed: Feed,
    config: &'a Config,
}

enum ChildContent<'a> {
    Text(&'a str),
    Func(Box<Fn(&'a LineToday<'a>, &mut Writer<Vec<u8>>) -> Result<(), XMLError> + 'a>),
    FuncItem(
        Box<Fn(&'a LineToday<'a>, &mut Writer<Vec<u8>>, usize) -> Result<(), XMLError> + 'a>,
        usize,
    ),
}

impl<'a> LineToday<'a> {
    pub fn build_xml(&'a self) -> Result<HttpResponse, Error> {
        let xml = self
            .into_xml()
            .map_err(|err| ErrorInternalServerError(err))?;

        Ok(HttpResponse::Ok().content_type("application/xml").body(xml))
    }

    fn into_xml(&'a self) -> Result<Vec<u8>, XMLError> {
        let mut writer = Writer::new_with_indent(Vec::new(), ' ' as u8, 2);
        let channel = self.feed.borrow_channel();

        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"articles")))?;

        let uuid = self.uuid();
        let time = self.time();
        self.xml_tag(&mut writer, b"UUID", ChildContent::Text(&uuid))?;
        self.xml_tag(&mut writer, b"time", ChildContent::Text(&time))?;

        for idx in 0..channel.items().len() {
            self.xml_tag(
                &mut writer,
                b"article",
                ChildContent::FuncItem(Box::new(Self::article_xml), idx),
            )?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"articles")))?;

        Ok(writer.into_inner())
    }

    fn xml_tag(
        &'a self,
        writer: &mut Writer<Vec<u8>>,
        tag_name: &[u8],
        content: ChildContent<'a>,
    ) -> Result<(), XMLError> {
        writer.write_event(Event::Start(BytesStart::borrowed_name(tag_name)))?;
        match content {
            ChildContent::Text(txt) => {
                writer.write_event(Event::Text(BytesText::from_plain_str(txt)))?;
            }
            ChildContent::Func(f) => (*f)(self, writer)?,
            ChildContent::FuncItem(f, i) => (*f)(self, writer, i)?,
        }
        writer.write_event(Event::End(BytesEnd::borrowed(tag_name)))?;
        Ok(())
    }

    fn uuid(&'a self) -> String {
        let channel = self.feed.borrow_channel();
        let mut uuid = String::from(channel.title());
        uuid.push_str(channel.last_build_date().unwrap_or(""));
        uuid.chars()
            .filter(|&ch| {
                (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9')
            })
            .take(30)
            .collect()
    }

    fn time(&'a self) -> String {
        let channel = self.feed.borrow_channel();
        (DateTime::parse_from_rfc2822(channel.last_build_date().unwrap_or(""))
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now())
            .timestamp() * 1000)
            .to_string()
    }

    fn article_xml(&'a self, writer: &mut Writer<Vec<u8>>, idx: usize) -> Result<(), XMLError> {
        self.xml_tag(writer, b"ID", ChildContent::Text(""))?;
        Ok(())
    }
}

pub fn from<'l>(feed: Feed, config: &'l Config) -> LineToday {
    LineToday {
        feed: feed,
        config: config,
    }
}
