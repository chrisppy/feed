// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! Implementation of `Feed`.


use Feed;
use channels::Channel;
use utils::writer_utils;


impl Feed
{
    /// Get the `Channel` that exists under `Feed`.
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
    /// let channels = feed.channel();
    /// ```
    pub fn channel(self) -> Channel
    {
        self.channel
    }


    /// Convert the `Feed` to XML.
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
    /// let xml = feed.to_xml();
    /// ```
    pub fn to_xml(&self) -> Vec<u8>
    {
        writer_utils::write(&self.channel)
    }
}
