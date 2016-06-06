// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `Feed`.

use atom::AtomFeed;
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
    pub fn channel(self) -> Option<Channel> {
        self.channel
    }


    /// Get the `AtomFeed` that exists under `Feed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// let atom_feed = AtomFeedBuilder::new()
    ///     .id("http://newrustacean.com/show_notes/e014/")
    ///     .title("e014: Stringing things along")
    ///     .updated("2014-11-28T12:00:09+00:00")
    ///     .finalize();
    ///
    /// let feed = FeedBuilder::new().feed(atom_feed).finalize();
    /// let out_feed = feed.feed();
    /// ```
    pub fn feed(self) -> Option<AtomFeed> {
        self.feed
    }


    /// Convert the `Feed` to XML.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::rss::{Channel, ChannelBuilder};
    ///
    /// let channel = ChannelBuilder::new().finalize();
    ///
    /// let feed = FeedBuilder::new().channel(channel).finalize();
    /// let xml = feed.xml();
    /// ```
    pub fn xml(&self) -> Vec<u8> {
        FeedWriter::new(self.channel.clone()).xml()
    }
}
