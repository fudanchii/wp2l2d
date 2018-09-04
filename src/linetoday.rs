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
use std::collections::hash_map::DefaultHasher;

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

    fn to_datetime(&self, date_str: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc2822(date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now())
    }

    fn to_unix_time_millis(&self, date_str: &str) -> String {
        (self.to_datetime(date_str).timestamp() * 1000).to_string()
    }

    fn time(&'a self) -> String {
        let channel = self.feed.borrow_channel();
        self.to_unix_time_millis(channel.last_build_date().unwrap_or(""))
    }

    fn id_for(&self, idx: usize) -> String {
        let channel = self.feed.borrow_channel();
        let item = channel.items()[idx];
        let indexed_title = String::new();
        let hasher = DefaultHasher::new();

        hasher.write(item.link().unwrap_or_else(|| {
            indexed_title.push_str(idx.to_string());
            indexed_title.push_str(channel.title());
            indexed_title
        }));
        hasher.finish().to_string();
    }

    fn language(&self) -> String {
        let channel = self.feed.borrow_channel();

        self.config
            .line_lang
            .unwrap_or_else(|| channel.language().unwrap_or("en").chars().take(2).collect())
    }

    fn start_pub_date(&self, idx: usize) -> String {
        let channel = self.feed.borrow_channel();
        let item = channel.items()[idx];

        self.to_unix_time_millis(item.pub_date().unwrap_or(""))
    }

    fn end_pub_date(&self, idx: usize) -> String {
        let channel = self.feed.borrow_channel();
        let item = channel.items()[idx];

        let end_date = self.to_datetime(item.pub_date().unwrap_or(""));
        ((end_date + ::time::Duration::days(14)).timestamp() * 1000).to_string()
    }

    fn article_xml(&'a self, writer: &mut Writer<Vec<u8>>, idx: usize) -> Result<(), XMLError> {
        let article_id = self.id_for(idx);
        let language = self.language();
        let native_country = self.config.line_native_country;
        let start_pub = self.start_pub_date(idx);
        let end_pub = self.end_pub_date(idx);
        let channel = self.feed.borrow_channel();
        let item = channel.items()[idx];
        let title = item.title().unwrap_or(channel.title());
        let category = item.categories()[0].name();
        let author = item.author().unwrap_or(channel.title());
        let source = item.link().unwrap_or(channel.link());

        self.xml_tag(writer, b"ID", ChildContent::Text(&article_id))?;
        self.xml_tag(
            writer,
            b"nativeCountry",
            ChildContent::Text(&native_country),
        )?;
        self.xml_tag(writer, b"language", ChildContent::Text(&language))?;

        if Some(pub_cnt) = self.config.line_pub_to_country {
            self.xml_tag(
                writer,
                b"publishCountries",
                ChildContent::Func(Box::new(Self::publish_to_country_xml)),
            )?;
        }

        if Some(excl_cnt) = self.config.line_excl_from_country {
            self.xml_tag(
                writer,
                b"excludedCountries",
                ChildContent::Func(Box::new(Self::exclude_from_country_xml)),
            )?;
        }

        self.xml_tag(writer, b"startYmdtUnix", ChildContent::Text(&start_pub))?;
        self.xml_tag(writer, b"endYmdtUnix", ChildContent::Text(&end_pub))?;

        self.xml_tag(writer, b"title", ChildContent::Text(&title))?;
        self.xml_tag(writer, b"category", ChildContent::Text(&category))?;
        self.xml_tag(writer, b"publishTimeUnix", ChildContent::Text(&start_pub))?;
        self.xml_tag(writer, b"contentType", ChildContent::Text("0"))?;

        self.xml_tag(
            writer,
            b"contents",
            childContent::FuncItem(Box::new(Self::content_xml), idx),
        )?;

        self.xml_tag(writer, b"author", childContent::Text(&author))?;
        self.xml_tag(writer, b"sourceUrl", childContent::Text(&source))?;

        Ok(())
    }

    fn content_xml(&'a self, writer: &mut Writer<Vec<u8>>, idx: usize) -> Result<(), XMLError> {}

    fn publish_to_country_xml(&'a self, writer: &mut Writer<Vec<u8>>) -> Result<(), XMLError> {}

    fn exclude_from_country_xml(&'a self, writer: &mut Writer<Vec<u8>>) -> Result<(), XMLError> {}
}

pub fn from<'l>(feed: Feed, config: &'l Config) -> LineToday {
    LineToday {
        feed: feed,
        config: config,
    }
}
