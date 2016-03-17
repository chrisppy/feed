// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
pub mod feedio;
pub mod rss;
mod util;

extern crate chrono;
extern crate curl;
extern crate log;
extern crate quick_xml;
extern crate url;

use curl::http;
use feedio::{FeedReader, FeedWriter};
use rss::Channel;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use url::Url;

#[derive(Clone)]
pub struct Feed {
    channel: Channel,
}


impl Feed {
    pub fn channel(self) -> Channel {
        self.channel
    }


    pub fn to_xml(&self) -> String {
        let feed_writer = FeedWriter::new(self.channel.clone());
        feed_writer.xml()
    }
}

#[derive(Default)]
pub struct FeedBuilder {
    channel: Channel,
}


impl FeedBuilder {
    pub fn new() -> FeedBuilder {
        FeedBuilder::default()
    }


    pub fn channel_from_url(&mut self, url: &str) -> &mut FeedBuilder {
        let url_with_ext = url.to_string() + ".xml";
        let feed_url = Url::parse(&url_with_ext).unwrap();
        let response = http::handle().get(feed_url.serialize()).exec().unwrap();
        let body = response.get_body();
        let feed = str::from_utf8(body).unwrap().to_string();
        let feed_reader = FeedReader::new(Some(feed.to_string()));
        self.channel = feed_reader.channel();
        self
    }


    pub fn channel_from_xml(&mut self, path_str: &str) -> &mut FeedBuilder {
        let path = match fs::canonicalize(path_str) {
            Ok(result) => result,
            Err(err)   => {panic!("Error: {}", err);},
        };
        let mut file = match File::open(path) {
            Ok(f)    => f,
            Err(err) => {panic!("Error: {}", err);},
        };
        let mut feed = String::new();
        match file.read_to_string(&mut feed) {
            Ok(result)    => (print!("Result: {}", result)),
            Err(err) => {panic!("Error: {}", err);},
        };
        let feed_reader = FeedReader::new(Some(feed.to_string()));
        self.channel = feed_reader.channel();
        self
    }


    pub fn channel(&mut self, channel: Channel) -> &mut FeedBuilder {
        self.channel = channel;
        self
    }


    pub fn finalize(&self) -> Feed {
        Feed {
            channel: self.channel.clone(),
        }
    }
}
