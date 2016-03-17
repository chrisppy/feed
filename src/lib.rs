// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

/// # Crate feed
///
/// ## feed 1.0.0
/// This Library is for parsing through a rss field and creating a `Feed` 
/// struct containing all elements of a `Channel` based on the rss spec.
///
/// ### Usage
/// Put this in your Cargo.toml:
/// ```Toml
/// [dependencies]
/// feedreader="1.0.0"
/// ```
/// And put this in your crate root:
/// ```
/// extern crate feed;
/// ```

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
use std::str;
use url::Url;

/// This `Feed` struct contains all the items that exist for the feeds.
#[derive(Clone)]
pub struct Feed {
    channel: Channel,
}


impl Feed {
    /// Get the `Channel` that exists under `Feed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::rss::Channel;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// let channel = feed.channel;
    /// ```
    pub fn channel(self) -> Channel {
        self.channel
    }


    /// Convert the `Feed` to XML.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::rss::Channel;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// unimplemented!()
    /// ```
    pub fn to_xml(&self) -> String {
        let feed_writer = FeedWriter::new(self.channel.clone());
        feed_writer.xml()
    }
}

/// This `FeedBuilder` struct creates the Feed struct from url, file, or &str.
#[derive(Default)]
pub struct FeedBuilder {
    channel: Channel,
}


impl FeedBuilder {
    /// Construct a new `FeedBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let feed_builder = FeedBuilder::new();
    /// ```
    pub fn new() -> FeedBuilder {
        FeedBuilder::default()
    }

    
    /// Construct a new `FeedBuilder` from a `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::rss::{Channel, ChannelBuilder};
    ///
    /// let channel = ChannelBuilder::new().finalize();
    /// let feed_builder = FeedBuilder::new().from_channel(channel);
    /// ```
    pub fn from_channel(&mut self, channel: Channel) -> &mut FeedBuilder {
        self.channel = channel;
        self
    }


    /// Construct a new `FeedBuilder` from a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let feed = "<rss><channel><title>The Linux Action Show! OGG</title></channel></rss>";
    /// let feed_builder = FeedBuilder::new().from_str(&feed);
    /// ```
    pub fn from_str(&mut self, feed: &str) -> &mut FeedBuilder {
        let feed_reader = FeedReader::new(Some(feed.to_string()));
        self.channel = feed_reader.channel();
        self
    }
    
    
    /// Construct a new `FeedBuilder` from a `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use url::Url;
    ///
    /// let url = "http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml";
    /// let feed_builder = FeedBuilder::new().from_str(&url);
    /// ```
    pub fn from_url(&mut self, feed_url: Url) -> &mut FeedBuilder {
        if !feed_url.serialize().as_str().ends_with(".xml") {
            panic!("Error: Url must end with .xml");
        }
        let response = match http::handle().get(feed_url.serialize()).exec() {
            Ok(resp) => resp,
            Err(err) => panic!("Response Error: {}", err),
        };
        let body = response.get_body();
        let feed_str = match str::from_utf8(body) {
            Ok(resp) => resp,
            Err(err) => panic!("from_utf8 Error: {}", err),
        };
        let feed_reader = FeedReader::new(Some(feed_str.to_string()));
        self.channel = feed_reader.channel();
        self
    }


    /// Construct the `Feed` from the `FeedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let feed = FeedBuilder::new().finalize);
    /// ```
    pub fn finalize(&self) -> Feed {
        Feed {
            channel: self.channel.clone(),
        }
    }
}
