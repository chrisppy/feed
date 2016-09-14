// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `FeedBuilder`.

use atom::AtomFeed;
use curl::http;
use errors;
use Feed;
use FeedBuilder;
use feedio::{ChannelReader, FeedReader};
use rss::{Channel, ChannelBuilder};
use std::str;
use url::Url;

impl FeedBuilder {
    /// Construct a new `FeedBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// FeedBuilder::new();
    /// ```
    pub fn new() -> FeedBuilder {
        let channel = ChannelBuilder::default().finalize();
        let feed = None;
        FeedBuilder {
            channel: channel,
            feed: feed,
        }
    }


    /// Construct a new `FeedBuilder` from a `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::rss::{Channel, ChannelBuilder};
    ///
    /// let channel = ChannelBuilder::new()
    ///     .title("The Linux Action Show")
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .description("Description")
    ///     .finalize();
    /// FeedBuilder::new().channel(channel);
    /// ```
    pub fn channel(&mut self, channel: Channel) -> &mut FeedBuilder {
        self.channel = channel;
        self
    }


    /// Construct a new `FeedBuilder` from a `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .title("The Linux Action Show")
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .description("Description")
    ///     .finalize();
    ///
    /// let text = TextBuilder::new().text("e014: Stringing things along").finalize();
    ///
    /// let atom_feed = AtomFeedBuilder::new()
    ///     .id("http://newrustacean.com/show_notes/e014/")
    ///     .title(text)
    ///     .updated("2014-11-28T12:00:09+00:00")
    ///     .finalize();
    /// FeedBuilder::new().channel(channel).feed(Some(atom_feed));
    /// ```
    pub fn feed(&mut self, feed: AtomFeed) -> &mut FeedBuilder {
        self.feed = Some(feed);
        self
    }


    /// Construct a new `FeedBuilder` from a `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::FeedBuilder;
    ///
    /// fn main() {
    ///     let url = "http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml";
    ///     let feed = FeedBuilder::new().read_from_url(url).finalize();
    ///     let channel = feed.channel();
    ///
    ///     let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    ///         + "Action Show! A show that covers everything geeks care about "
    ///         + "in the computer industry. Get a solid dose of Linux, "
    ///         + "gadgets, news events and much more!";
    ///
    ///     let generator = "Feeder 2.5.12(2294); ".to_owned()
    ///         + "Mac OS X Version 10.9.5 (Build 13F34) "
    ///         + "http://reinventedsoftware.com/feeder/";
    ///
    ///     assert_eq!("The Linux Action Show! OGG".to_owned(), channel.title());
    ///     assert_eq!("http://www.jupiterbroadcasting.com/".to_owned(), channel.link().into_string());
    ///     assert_eq!(description.as_ref(), channel.description());
    ///     assert_eq!(Some(generator), channel.generator());
    ///     assert_eq!(Some("http://blogs.law.harvard.edu/tech/rss".to_owned()), channel.docs());
    ///     assert_eq!(Some("en".to_owned()), channel.language());
    ///     assert!(channel.copyright().is_none());
    ///     assert!(channel.managing_editor().is_none());
    ///     assert!(channel.web_master().is_none());
    /// }
    /// ```
    pub fn read_from_url(&mut self, feed: &str) -> &mut FeedBuilder {
        if !feed.ends_with(".xml") {
            panic!(errors::missing_xml_error());
        }

        let feed_url = Url::parse(feed).expect(errors::url_parse_error(feed).as_str());

        let response = http::handle()
            .get(feed_url.into_string())
            .exec()
            .expect(errors::response_error());
        let body = response.get_body();
        let feed_str = str::from_utf8(body).expect(errors::utf8_to_str_error());
        debug!("feed xml:{}", feed_str);
        self.channel = ChannelReader::new(feed_str).channel();
        if feed_str.contains("http://www.w3.org/2005/Atom/") {
            self.feed = Some(FeedReader::new(feed_str).feed());
        } else {
            self.feed = None;
        }

        self
    }


    /// Construct the `Feed` from the `FeedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .title("The Linux Action Show")
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .description("Description")
    ///     .finalize();
    ///
    /// FeedBuilder::new().channel(channel).finalize();
    /// ```
    pub fn finalize(&self) -> Feed {
        Feed {
            channel: self.channel.clone(),
            feed: self.feed.clone(),
        }
    }
}
