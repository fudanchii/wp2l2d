#[macro_use]
extern crate serde_derive;
extern crate envy;

extern crate futures;
extern crate actix_web;
extern crate quick_xml;
extern crate rss;

pub mod config;
pub mod routes;

pub mod healthcheck;
pub mod linetoday;
pub mod wordpress;
