// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


#![doc(html_root_url = "https://docs.rs/feed/")]
#![deny(missing_docs)]


//! # feed 3.0
//!
//! This Library is for parsing through a channels field and creating a `Feed`
//! struct containing all elements of a `Channel` based on the channels spec.
//!
//! ## Usage
//! Put this in your Cargo.toml:
//!
//! ```Toml
//! [dependencies]
//! feed = "3.0"
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
//! extern crate rss;
//! extern crate feed;
//!
//! use feed::channels::{FromUrl, ChannelGetters};
//! use rss::Channel;
//!
//! fn main()
//! {
//!     let url = "https://feedpress.me/usererror.xml";
//!     let channel = Channel::from_url(url).unwrap();
//!     println!("Feed Title: {:?}", channel.title());
//! }
//! ```
//!
//! ### Writing Feeds
//!
//! ```
//! extern crate feed;
//!
//! use feed::channels::ChannelBuilder;
//!
//! fn main()
//! {
//!     let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
//!         + "Action Show! A show that covers everything geeks care about in "
//!         + "the computer industry. Get a solid dose of Linux, gadgets, news "
//!         + "events and much more!";
//!
//!     let channel = ChannelBuilder::new()
//!         .title("The Linux Action Show! OGG")
//!         .link("http://www.jupiterbroadcasting.com")
//!         .description(description.as_ref())
//!         .finalize()
//!         .unwrap();
//!
//!     println!("Feed: {:?}", channel.to_string());
//! }
//! ```


extern crate chrono;
extern crate curl;
extern crate mime;
extern crate rss;
extern crate url;


pub mod channels;
pub mod enums;
mod utils;
