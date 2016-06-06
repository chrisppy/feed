// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The feed can be converted to xml.

pub mod channel_reader;
pub mod feed_reader;
pub mod feed_writer;

use atom::AtomFeed;
use rss::Channel;

/// This `ChannelReader` struct parses the xml feed to the `Channel`.
pub struct ChannelReader {
    channel: Channel,
}


/// This `FeedReader` struct parses the xml feed to the `AtomFeed`.
pub struct FeedReader {
    feed: AtomFeed,
}


/// This `FeedWriter` struct creates the xml from the `Channel`.
#[derive(Default)]
pub struct FeedWriter {
    xml: Vec<u8>,
}
