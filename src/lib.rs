// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod feedreader;
mod rss;

extern crate chrono;
extern crate curl;
extern crate quick_xml;
extern crate url;

use rss::Channel;
use curl::http;
use std::str;
use url::Url;
use feedreader::FeedReader;

#[derive(Clone)]
pub struct Feed {
    channel: Channel,
}


impl Feed {
    pub fn get_channel(self) -> Channel {
        self.channel
    }


    pub fn to_xml(&self) {
        panic!("Not Yet Implemented");
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
        let feed_url = Url::parse(url).unwrap();
        let response = http::handle().get(feed_url.serialize()).exec().unwrap();
        let body = response.get_body();
        let feed = str::from_utf8(body).unwrap().to_string();
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
