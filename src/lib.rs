// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! # feed 2.0
//!
//! This Library is for parsing through a rss field and creating a `Feed`
//! struct containing all elements of a `Channel` based on the rss spec.
//!
//! ## Usage
//! Put this in your Cargo.toml:
//!
//! ```Toml
//! [dependencies]
//! feed = "2.0"
//! ```
//!
//! And put this in your crate root:
//!
//! ```
//! extern crate feed;
//! ```
//!
//! ## Examples
//!
//! ### Reading Feeds
//!
//! ```
//! extern crate feed;
//!
//! use feed::FeedBuilder;
//!
//! fn main() {
//!     let url = "http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml";
//!     let feed = FeedBuilder::new().read_from_url(url).finalize();
//!     let channel = feed.channel().unwrap();
//!     println!("Title: {}", channel.title());
//! }
//! ```
//!
//! ### Writing Feeds
//!
//! ```
//! extern crate feed;
//!
//! use feed::FeedBuilder;
//! use feed::rss::ChannelBuilder;
//!
//! fn main() {
//!
//!     let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
//!         + "Action Show! A show that covers everything geeks care about in "
//!         + "the computer industry. Get a solid dose of Linux, gadgets, news "
//!         + "events and much more!";
//!
//!     let channel = ChannelBuilder::new()
//!             .title("The Linux Action Show! OGG")
//!             .link("http://www.jupiterbroadcasting.com")
//!             .description(description.as_ref())
//!             .finalize();
//!     let feed = FeedBuilder::new().channel(channel).finalize();
//!     let xml = feed.xml();
//!     println!("Feed: {:?}", xml);
//! }
//! ```

#![doc(html_root_url = "http://feed.red-oxide.org")]

#![deny(missing_docs)]

pub mod atom;
pub mod feed;
pub mod feed_builder;
pub mod feedio;
pub mod itunes;
pub mod media;
pub mod rss;
pub mod util;
pub mod errors;

extern crate chrono;
extern crate curl;
extern crate quick_xml;
extern crate url;

#[macro_use]
extern crate log;

use atom::AtomFeed;
use rss::Channel;

/// This `Feed` struct contains all the items that exist for the feeds.
#[derive(Clone)]
pub struct Feed {
    channel: Option<Channel>,
    feed: Option<AtomFeed>,
}


/// This `FeedBuilder` struct creates the Feed struct from url, file, or &str.
#[derive(Default)]
pub struct FeedBuilder {
    channel: Option<Channel>,
    feed: Option<AtomFeed>,
}
