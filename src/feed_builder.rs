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
    ///     .finalize()
    ///     .unwrap();
    /// FeedBuilder::channel(channel).unwrap();
    /// ```
    pub fn channel(channel: Channel) -> Result<Feed, String>
    {
        Ok(Feed { channel: channel })
    }


    /// Construct a new `FeedBuilder` from xml`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    ///              <rss version="2.0">
    ///                  <channel>
    ///                      <title>The Linux Action Show! OGG</title>
    ///                      <link>http://www.jupiterbroadcasting.com</link>
    ///                      <description>Linux</description>
    ///                  </channel>
    ///              </rss>"#;
    ///
    /// FeedBuilder::from_xml(xml).unwrap();
    /// ```
    pub fn from_xml(xml: &str) -> Result<Feed, String>
    {
        FeedBuilder::channel(reader_utils::read(xml)?)
    }


    /// Construct a new `FeedBuilder` from a `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let url = "https://feedpress.me/usererror.xml";
    /// let feed = FeedBuilder::read_from_url(url).unwrap();
    /// feed.channel();
    /// ```
    pub fn read_from_url(feed_str: &str) -> Result<Feed, String>
    {
        let feed_url = string_utils::str_to_url(feed_str)?;
        let mut xml = Vec::new();
        let mut handle = Easy::new();

        let url = handle.url(feed_url.into_string().as_str());
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                                        xml.extend_from_slice(data);
                                        Ok(data.len())
                                    })
                .unwrap();
            transfer.perform().unwrap();
        }

        if url.is_err()
        {
            return Err(format!("Error: {:?}", url.unwrap_err()));
        }

        let content_type = match handle.content_type()
        {
            Ok(val) => val.unwrap(),
            Err(err) => return Err(format!("Error: {}", err)),
        };

        if !content_type.contains("xml")
        {
            return Err("Url must end with .xml".to_owned());
        }

        match String::from_utf8(xml)
        {
            Ok(val) => FeedBuilder::from_xml(val.as_str()),
            Err(err) => Err(format!("Error: {}", err)),
        }
    }
}
