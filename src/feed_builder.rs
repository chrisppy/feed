// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! Implementation of `FeedBuilder`.


use curl::easy::Easy;
use errors;
use Feed;
use FeedBuilder;
use utils::reader_utils;
use channels::Channel;
use std::str;
use url::Url;


impl FeedBuilder {
    /// Construct a new `FeedBuilder` from a `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new().finalize();
    /// FeedBuilder::channel(channel);
    /// ```
    pub fn channel(channel: Channel) -> FeedBuilder {
        FeedBuilder { channel: channel }
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
    ///     let url = "http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml";
    ///     let feed = FeedBuilder::read_from_url(url).finalize();
    ///     let channels = feed.channel();
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
    ///     assert_eq!("The Linux Action Show! OGG".to_owned(), channels.title());
    ///     assert_eq!("http://www.jupiterbroadcasting.com".to_owned(), channels.link());
    ///     assert_eq!(description.as_ref(), channels.description());
    ///     assert_eq!(Some(generator), channels.generator());
    ///     assert_eq!(Some("http://blogs.law.harvard.edu/tech/rss".to_owned()), channels.docs());
    ///     assert_eq!(Some("en".to_owned()), channels.language());
    ///     assert!(channels.copyright().is_none());
    ///     assert!(channels.managing_editor().is_none());
    ///     assert!(channels.web_master().is_none());
    /// }
    /// ```
    pub fn read_from_url(feed_str: &str) -> FeedBuilder {
        let feed_url = Url::parse(feed_str).expect(errors::str_to_url_error().as_str());
        let mut dst = Vec::new();
        let mut handle = Easy::new();

        let url = handle.url(feed_url.into_string().as_str());
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        if url.is_err() {
            panic!(errors::url_error());
        }

        let content_type = handle.content_type()
            .expect(errors::content_type_error().as_str())
            .unwrap();

        if !content_type.contains("xml") {
            panic!(errors::missing_xml_error());
        }

        let feed_string = String::from_utf8(dst).expect(errors::utf8_to_str_error().as_str());

        FeedBuilder { channel: reader_utils::read(feed_string.as_str()) }
    }


    /// Construct the `Feed` from the `FeedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new().finalize();
    /// let feed = FeedBuilder::channel(channel).finalize();
    /// ```
    pub fn finalize(&self) -> Feed {
        Feed { channel: self.channel.clone() }
    }
}
