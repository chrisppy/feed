// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! Implementation of `FeedBuilder`.


use Feed;
use FeedBuilder;
use channels::Channel;
use curl::easy::Easy;
use errors;
use std::str;
use utils::{reader_utils, string_utils};


impl FeedBuilder
{
    /// Construct a new `FeedBuilder` from a `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// FeedBuilder::channel(channel);
    /// ```
    pub fn channel(channel: Channel) -> FeedBuilder
    {
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
    ///
    /// fn main() {
    ///     let url = "https://feedpress.me/usererror.xml";
    ///     let feed = FeedBuilder::read_from_url(url).finalize();
    ///     feed.channel();
    /// }
    /// ```
    pub fn read_from_url(feed_str: &str) -> FeedBuilder
    {
        let feed_url = string_utils::str_to_url(feed_str, "read_from_url");
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

        if url.is_err()
        {
            panic!(errors::url_error());
        }

        let content_type = handle.content_type()
            .expect(errors::content_type_error().as_str())
            .unwrap();

        if !content_type.contains("xml")
        {
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
    /// let channel = ChannelBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let feed = FeedBuilder::channel(channel).finalize();
    /// ```
    pub fn finalize(&self) -> Feed
    {
        Feed { channel: self.channel.clone() }
    }
}
