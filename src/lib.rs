// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


#![doc(html_root_url = "https://docs.rs/feed/")]
#![deny(missing_docs)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


//! # feed 2.0
//!
//! This Library is for parsing through a channels field and creating a `Feed`
//! struct containing all elements of a `Channel` based on the channels spec.
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
//! extern crate url;
//!
//! use feed::FeedBuilder;
//!
//! fn main() {
//!     let url_str = "https://feedpress.me/usererror.xml";
//!     let feed = FeedBuilder::read_from_url(url_str).finalize();
//!     let channel = feed.channel();
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
//! use feed::channels::ChannelBuilder;
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
//!     let feed = FeedBuilder::channel(channel).finalize();
//!     let xml = feed.to_xml();
//!     println!("Feed: {:?}", xml);
//! }
//! ```


extern crate chrono;
extern crate curl;
extern crate rss;
extern crate url;


pub mod channels;
pub mod enums;
mod errors;
pub mod feed;
pub mod feed_builder;
mod utils;


use channels::Channel;


/// This `Feed` struct contains all the items that exist for the feeds.
#[derive(Clone)]
pub struct Feed {
    channel: Channel,
}


/// This `FeedBuilder` struct creates the Feed struct from url, file, or &str.
#[derive(Clone)]
pub struct FeedBuilder {
    channel: Channel,
}
