// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! Implementation of `Feed`.


use Feed;
use utils::writer_utils;
use channels::Channel;


impl Feed {
    /// Get the `Channel` that exists under `Feed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new().finalize();
    /// let feed = FeedBuilder::channel(channel).finalize();
    /// let channels = feed.channel();
    /// ```
    pub fn channel(self) -> Channel {
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
    /// let channel = ChannelBuilder::new().finalize();
    /// let feed = FeedBuilder::channel(channel).finalize();
    /// let xml = feed.to_xml();
    /// ```
    pub fn to_xml(&self) -> Vec<u8> {
        writer_utils::write(self.channel.clone())
    }
}
