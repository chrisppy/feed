// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! # feed 1.0.3
//!
//! This Library is for parsing through a rss field and creating a `Feed`
//! struct containing all elements of a `Channel` based on the rss spec.
//!
//! ## Usage
//! Put this in your Cargo.toml:
//!
//! ```Toml
//! [dependencies]
//! feedreader = "1.0.3"
//! ```
//!
//! And put this in your crate root:
//!
//! ```
//! extern crate feed;
//! ```
//!
//! ## Examples
//!
//! ```
//! extern crate feed;
//! extern crate url;
//!
//! use feed::FeedBuilder;
//! use url::Url;
//!
//! fn main() {
//!     let url = Url::parse("http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml").expect("Url parse Error");
//!     let feed = FeedBuilder::new().read_from_url(url).finalize();
//!     let channel = feed.channel();
//! }
//! ```

#![doc(html_root_url = "https://red-oxide.github.io/feed/")]

#![deny(missing_docs)]

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
    /// let channel = feed.channel();
    /// ```
    pub fn channel(self) -> Channel {
        self.channel
    }


    /// Convert the `Feed` to XML.
    ///
    /// Not Yet Implemented!
    ///
    /// To be added in 1.1.0
    pub fn to_xml(&self) -> String {
        FeedWriter::new().xml()
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
    /// let feed = FeedBuilder::new().finalize();
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
    /// let feed = FeedBuilder::new().channel(channel).finalize();
    /// ```
    pub fn channel(&mut self, channel: Channel) -> &mut FeedBuilder {
        self.channel = channel;
        self
    }


    /// Construct a new `FeedBuilder` from a `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    /// extern crate url;
    ///
    /// use feed::FeedBuilder;
    /// use url::Url;
    /// fn main() {
    ///     let url = Url::parse("http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml").expect("Url parse Error");
    ///     let feed = FeedBuilder::new().read_from_url(url).finalize();
    ///     let channel = feed.channel();
    ///     assert_eq!("The Linux Action Show! OGG".to_owned(), channel.title());
    ///     assert_eq!("http://www.jupiterbroadcasting.com".to_owned(), channel.link());
    ///     assert_eq!("Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!".to_owned(), channel.description());
    ///     assert_eq!(Some("Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_owned()), channel.generator());
    ///     assert_eq!(Some("http://blogs.law.harvard.edu/tech/rss".to_owned()), channel.docs());
    ///     assert_eq!(Some("en".to_owned()), channel.language());
    ///     assert!(channel.copyright().is_none());
    ///     assert!(channel.managing_editor().is_none());
    ///     assert!(channel.web_master().is_none());
    /// }
    /// ```
    pub fn read_from_url(&mut self, feed_url: Url) -> &mut FeedBuilder {
        if !feed_url.serialize().as_str().ends_with(".xml") {
            panic!("Error: Url must end with .xml");
        }
        let response = http::handle().get(feed_url.serialize()).exec().expect("Response Error");
        let body = response.get_body();
        let feed_str = str::from_utf8(body).expect("from_utf8 Error");
        self.channel = FeedReader::new(Some(feed_str.to_owned())).channel();
        self
    }


    /// Construct the `Feed` from the `FeedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// ```
    pub fn finalize(&self) -> Feed {
        Feed {
            channel: self.channel.clone(),
        }
    }
}
