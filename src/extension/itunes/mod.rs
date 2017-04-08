// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! All the structs for itunes.


pub mod itunes_category_getters;
pub mod itunes_category_builder;
pub mod itunes_owner_getters;
pub mod itunes_owner_builder;
pub mod itunes_item_extension_getters;
pub mod itunes_item_extension_builder;
pub mod itunes_channel_extension_getters;
pub mod itunes_channel_extension_builder;


use rss::extension::itunes::{ITunesCategory, ITunesOwner};


/// The Getter functions for `ITunesCategory`
pub trait ITunesCategoryGetters
{
    /// Get the text that exists under `ITunesCategory`.
    fn text(&self) -> String;

    /// Get the optional subcategory that exists under `ITunesCategory`.
    fn subcategory(&self) -> Option<Box<ITunesCategory>>;
}


/// This `ITunesCategoryBuilder` struct creates the `ITunesCategory`.
#[derive(Clone, Default)]
pub struct ITunesCategoryBuilder
{
    text: String,
    subcategory: Option<Box<ITunesCategory>>,
}


/// The Getter functions for `ITunesOwner`
pub trait ITunesOwnerGetters
{
    /// Get the optional name that exists under `ITunesOwner`.
    fn name(&self) -> Option<String>;

    /// Get the optional email that exists under `ITunesOwner`.
    fn email(&self) -> Option<String>;
}


/// This `ITunesOwnerBuilder` struct creates the `ITunesOwner`.
#[derive(Clone, Default)]
pub struct ITunesOwnerBuilder
{
    name: Option<String>,
    email: Option<String>,
}


/// The Getter functions for `ITunesChannelExtension`
pub trait ITunesChannelExtensionGetters
{
    /// Get the optional author that exists under `ITunesChannelExtension`.
    fn author(&self) -> Option<String>;

    /// Get the optional block that exists under `ITunesChannelExtension`.
    fn block(&self) -> Option<String>;

    /// Get the categories that exists under `ITunesChannelExtension`.
    fn categories(&self) -> Vec<ITunesCategory>;

    /// Get the optional image that exists under `ITunesChannelExtension`.
    fn image(&self) -> Option<String>;

    /// Get the optional explicit that exists under `ITunesChannelExtension`.
    fn explicit(&self) -> Option<String>;

    /// Get the optional complete that exists under `ITunesChannelExtension`.
    fn complete(&self) -> Option<String>;

    /// Get the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    fn new_feed_url(&self) -> Option<String>;

    /// Get the optional owner that exists under `ITunesChannelExtension`.
    fn owner(&self) -> Option<ITunesOwner>;

    /// Get the optional subtitle that exists under `ITunesChannelExtension`.
    fn subtitle(&self) -> Option<String>;

    /// Get the optional summary that exists under `ITunesChannelExtension`.
    fn summary(&self) -> Option<String>;

    /// Get the optional keywords that exists under `ITunesChannelExtension`.
    fn keywords(&self) -> Option<String>;
}


/// This `ITunesChannelExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
#[derive(Clone, Default)]
pub struct ITunesChannelExtensionBuilder
{
    author: Option<String>,
    block: Option<String>,
    categories: Vec<ITunesCategory>,
    image: Option<String>,
    explicit: Option<String>,
    complete: Option<String>,
    new_feed_url: Option<String>,
    owner: Option<ITunesOwner>,
    subtitle: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
}


/// The Getter functions for `ITunesItemExtension`
pub trait ITunesItemExtensionGetters
{
    /// Get the optional author that exists under `ITunesItemExtension`.
    fn author(&self) -> Option<String>;

    /// Get the optional block that exists under `ITunesItemExtension`.
    fn block(&self) -> Option<String>;

    /// Get the optional image that exists under `ITunesItemExtension`.
    fn image(&self) -> Option<String>;

    /// Get the optional duration that exists under `ITunesItemExtension`.
    fn duration(&self) -> Option<String>;

    /// Get the optional explicit that exists under `ITunesItemExtension`.
    fn explicit(&self) -> Option<String>;

    /// Get the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    fn closed_captioned(&self) -> Option<String>;

    /// Get the optional order that exists under `ITunesItemExtension`.
    fn order(&self) -> Option<String>;

    /// Get the optional subtitle that exists under `ITunesItemExtension`.
    fn subtitle(&self) -> Option<String>;

    /// Get the optional summary that exists under `ITunesItemExtension`.
    fn summary(&self) -> Option<String>;

    /// Get the optional keywords that exists under `ITunesItemExtension`.
    fn keywords(&self) -> Option<String>;
}


/// field under 'Item'.
/// This `ITunesItemExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
#[derive(Clone, Default)]
pub struct ITunesItemExtensionBuilder
{
    author: Option<String>,
    block: Option<String>,
    image: Option<String>,
    duration: Option<String>,
    explicit: Option<String>,
    closed_captioned: Option<String>,
    order: Option<String>,
    subtitle: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
}
