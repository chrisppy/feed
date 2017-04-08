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
//! use feed::{FromUrl, ChannelGetters};
//! use rss::Channel;
//!
//! fn main()
//! {
//!     let url = "https://feedpress.me/usererror.xml";
//!
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
//! use feed::ChannelBuilder;
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
//!         .finalize().unwrap();
//!
//!     println!("Feed: {:?}", channel.to_string());
//! }
//! ```
//!
//! ### Validating Feeds
//!
//! ```
//! extern crate feed;
//!
//! use feed::ChannelBuilder;
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
//!         .validate().unwrap()
//!         .finalize().unwrap();
//!
//!     println!("Feed: {:?}", channel.to_string());
//! }
//! ```
//!
//! ```
//! extern crate rss;
//! extern crate feed;
//!
//! use feed::{FromUrl, Validate};
//! use rss::Channel;
//!
//! fn main()
//! {
//!     let url = "https://feedpress.me/usererror.xml";
//!
//!     let channel = Channel::from_url(url).unwrap();
//!     channel.validate().unwrap();
//! }
//! ```

extern crate chrono;
extern crate curl;
extern crate mime;
extern crate rss;
extern crate url;


mod enums;
pub mod extension;
pub mod channel;
mod utils;


use rss::{Category, Channel, Cloud, Enclosure, Guid, Image, Item, Source, TextInput};
use rss::extension::itunes::{ITunesChannelExtension, ITunesItemExtension};

/// The Getter functions for `Category`
pub trait CategoryGetters
{
    /// Get the category that exists under `Category`.
    fn name(&self) -> String;


    /// Get the optional domain that exists under `Category`.
    fn domain(&self) -> Option<String>;
}


/// This `CategoryBuilder` struct creates the `Category`.
#[derive(Clone, Default)]
pub struct CategoryBuilder
{
    name: String,
    domain: Option<String>,
}


/// From Url function for `Channel`
pub trait FromUrl
{
    /// Construct a `Channel` from a `Url`.
    fn from_url(url: &str) -> Result<Channel, String>;
}


/// Validate function for `Channel`
pub trait Validate
{
    /// Validate `Channel`
    fn validate(&self) -> Result<Channel, String>;
}


/// The Getter functions for `Channel`
pub trait ChannelGetters
{
    /// Get the title that exists under `Channel`.
    fn title(&self) -> String;

    /// Get the link that exists under `Channel`.
    fn link(&self) -> String;

    /// Get the description that exists under `Channel`.
    fn description(&self) -> String;

    /// Get the optional language that exists under `Channel`.
    fn language(&self) -> Option<String>;

    /// Get the optional copyright that exists under `Channel`.
    fn copyright(&self) -> Option<String>;

    /// Get the optional managing editor that exists under `Channel`.
    fn managing_editor(&self) -> Option<String>;

    /// Get the optional web master that exists under `Channel`.
    fn webmaster(&self) -> Option<String>;

    /// Get the optional pub date that exists under `Channel`.
    fn pub_date(&self) -> Option<String>;

    /// Get the optional last build date that exists under `Channel`.
    fn last_build_date(&self) -> Option<String>;

    /// Get the categories that exists under `Channel`.
    fn categories(&self) -> Vec<Category>;

    /// Get the optional generator that exists under `Channel`.
    fn generator(&self) -> Option<String>;

    /// Get the optional docs that exists under `Channel`.
    fn docs(&self) -> Option<String>;

    /// Get the optional cloud that exists under `Channel`.
    fn cloud(&self) -> Option<Cloud>;

    /// Get the optional ttl that exists under `Channel`.
    fn ttl(&self) -> Option<String>;

    /// Get the optional image that exists under `Channel`.
    fn image(&self) -> Option<Image>;

    /// Get the optional text input that exists under `Channel`.
    fn text_input(&self) -> Option<TextInput>;

    /// Get the optional rating that exists under `Channel`.
    fn rating(&self) -> Option<String>;

    /// Get the skip hours that exists under `Channel`.
    fn skip_hours(&self) -> Vec<String>;

    /// Get the skip days that exists under `Channel`.
    fn skip_days(&self) -> Vec<String>;

    /// Get the items that exists under `Channel`.
    fn items(&self) -> Vec<Item>;

    /// Get the optional `ITunesChannelExtension` under `Channel`.
    fn itunes_ext(&self) -> Option<ITunesChannelExtension>;
}


/// This `ChannelBuilder` struct creates the `Channel`.
#[derive(Clone, Default)]
pub struct ChannelBuilder
{
    title: String,
    link: String,
    description: String,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    webmaster: Option<String>,
    pub_date: Option<String>,
    last_build_date: Option<String>,
    categories: Option<Vec<Category>>,
    generator: Option<String>,
    docs: Option<String>,
    cloud: Option<Cloud>,
    ttl: Option<i64>,
    image: Option<Image>,
    rating: Option<String>,
    text_input: Option<TextInput>,
    skip_hours: Option<Vec<i64>>,
    skip_days: Option<Vec<String>>,
    items: Option<Vec<Item>>,
    itunes_ext: Option<ITunesChannelExtension>,
}


/// The Getter functions for `Cloud`
pub trait CloudGetters
{
    /// Get the domain that exists under `Cloud`.
    fn domain(&self) -> String;

    /// Get the port that exists under `Cloud`.
    fn port(&self) -> String;

    /// Get the path that exists under `Cloud`.
    fn path(&self) -> String;

    /// Get the register procedure that exists under `Cloud`.
    fn register_procedure(&self) -> String;

    /// Get the protocol that exists under `Cloud`.
    fn protocol(&self) -> String;
}


/// This `CloudBuilder` struct creates the `Cloud`.
#[derive(Clone, Default)]
pub struct CloudBuilder
{
    domain: String,
    port: i64,
    path: String,
    register_procedure: String,
    protocol: String,
}


/// The Getter functions for `Enclosure`
pub trait EnclosureGetters
{
    /// Get the url that exists under `Enclosure`.
    fn url(&self) -> String;

    /// Get the length that exists under `Enclosure`.
    fn length(&self) -> String;

    /// Get the enclosure type that exists under `Enclosure`.
    fn mime_type(&self) -> String;
}


/// This `EnclosureBuilder` struct creates the `Enclosure`.
#[derive(Clone, Default)]
pub struct EnclosureBuilder
{
    url: String,
    length: i64,
    mime_type: String,
}


/// The Getter functions for `Guid`
pub trait GuidGetters
{
    /// Get the permalink that exists under `Guid`.
    fn is_permalink(&self) -> bool;

    /// Get the guid that exists under `Guid`.
    fn value(&self) -> String;
}


/// This `GuidBuilder` struct creates the `Guid`.
#[derive(Clone, Default)]
pub struct GuidBuilder
{
    is_permalink: Option<bool>,
    value: String,
}


/// The Getter functions for `Image`
pub trait ImageGetters
{
    /// Get the url that exists under `Image`.
    fn url(&self) -> String;

    /// Get the title that exists under `Image`.
    fn title(&self) -> String;

    /// Get the link that exists under `Image`.
    fn link(&self) -> String;

    /// Get the width that exists under `Image`.
    fn width(&self) -> Option<String>;

    /// Get the height that exists under `Image`.
    fn height(&self) -> Option<String>;

    /// Get the description that exists under `Image`.
    fn description(&self) -> Option<String>;
}


/// This `ImageBuilder` struct creates the `Image`.
#[derive(Clone, Default)]
pub struct ImageBuilder
{
    url: String,
    title: String,
    link: String,
    width: Option<i64>,
    height: Option<i64>,
    description: Option<String>,
}


/// The Getter functions for `Item`
pub trait ItemGetters
{
    /// Get the optional title that exists under `Item`.
    fn title(&self) -> Option<String>;

    /// Get the optional link that exists under `Item`.
    fn link(&self) -> Option<String>;

    /// Get the optional description that exists under `Item`.
    fn description(&self) -> Option<String>;

    /// Get the optional author that exists under `Item`.
    fn author(&self) -> Option<String>;

    /// Get the categories that exists under `Item`.
    fn categories(&self) -> Vec<Category>;

    /// Get the optional comments that exists under `Item`.
    fn comments(&self) -> Option<String>;

    /// Get the optional enclosure that exists under `Item`.
    fn enclosure(&self) -> Option<Enclosure>;

    /// Get the optional guid that exists under `Item`.
    fn guid(&self) -> Option<Guid>;

    /// Get the optional pub date that exists under `Item`.
    fn pub_date(&self) -> Option<String>;

    /// Get the optional source that exists under `Item`.
    fn source(&self) -> Option<Source>;

    /// Get the optional `ITunesItemExtension` under `Item`.
    fn itunes_ext(&self) -> Option<ITunesItemExtension>;
}


/// This `ItemBuilder` struct creates the `Item`.
#[derive(Clone, Default)]
pub struct ItemBuilder
{
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    author: Option<String>,
    categories: Option<Vec<Category>>,
    comments: Option<String>,
    enclosure: Option<Enclosure>,
    guid: Option<Guid>,
    pub_date: Option<String>,
    source: Option<Source>,
    itunes_ext: Option<ITunesItemExtension>,
}


/// The Getter functions for `Source`
pub trait SourceGetters
{
    /// Get the url that exists under `Source`.
    fn url(&self) -> String;

    /// Get the source that exists under `Source`.
    fn title(&self) -> Option<String>;
}


/// This `SourceBuilder` struct creates the `Source`.
#[derive(Clone, Default)]
pub struct SourceBuilder
{
    url: String,
    title: Option<String>,
}


/// The Getter functions for `TextInput`
pub trait TextInputGetters
{
    /// Get the title that exists under `TextInput`.
    fn title(&self) -> String;

    /// Get the description that exists under `TextInput`.
    fn description(&self) -> String;

    /// Get the name that exists under `TextInput`.
    fn name(&self) -> String;

    /// Get the link that exists under `TextInput`.
    fn link(&self) -> String;
}


/// This `TextInputBuilder` struct creates the `TextInput`.
#[derive(Clone, Default)]
pub struct TextInputBuilder
{
    title: String,
    description: String,
    name: String,
    link: String,
}
