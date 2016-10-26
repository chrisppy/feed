// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The feed can be converted to xml.

pub mod feed_reader;
pub mod feed_writer;

use rss::Channel;

/// This `FeedReader` struct parses the xml feed to the `Channel`.
pub struct FeedReader {
    channel: Channel,
}


/// This `FeedWriter` struct creates the xml from the `Channel`.
#[derive(Default)]
pub struct FeedWriter {
    xml: Vec<u8>,
}
