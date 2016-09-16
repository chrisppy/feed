// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `Feed`.

use Feed;
use feedio::FeedWriter;
use rss::Channel;

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
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::rss::Channel;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// let xml = feed.to_xml();
    /// ```
    pub fn to_xml(&self) -> Vec<u8> {
        FeedWriter::new(self.channel.clone()).xml()
    }
}
