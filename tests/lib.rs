// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod rss;

extern crate chrono;
extern crate feed;

use feed::{Feed, FeedBuilder};

#[test]
fn from_url() {
    unimplemented!()
}


#[test]
fn from_xml() {
    let feed = FeedBuilder::new().channel_from_xml("../rsc/sample.xml").finalize();
    let channel = feed.channel();
    assert_eq!("The Linux Action Show! OGG".to_string(), channel.title());
    assert_eq!("http://www.jupiterbroadcasting.com".to_string(), channel.link());
}
