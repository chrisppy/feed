// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! All the structs for itunes.


pub mod itunes_category;
pub mod itunes_category_builder;
pub mod itunes_owner;
pub mod itunes_owner_builder;
pub mod itunes_item_extension;
pub mod itunes_item_extension_builder;
pub mod itunes_channel_extension;
pub mod itunes_channel_extension_builder;


/// This `ITunesCategory` struct contains all the items that exist for the
/// category
/// field under `ITunesChannelExtension`.
#[derive(Clone)]
pub struct ITunesCategory
{
    text: String,
    subcategory: Option<Box<ITunesCategory>>,
}


/// This `ITunesCategoryBuilder` struct creates the `ITunesCategory`.
#[derive(Clone, Default)]
pub struct ITunesCategoryBuilder
{
    text: String,
    subcategory: Option<Box<ITunesCategory>>,
}


/// This `ITunesOwner` struct contains all the items that exist for the owner
/// field under `ITunesChannelExtension`.
#[derive(Clone)]
pub struct ITunesOwner
{
    name: Option<String>,
    email: Option<String>,
}


/// This `ITunesOwnerBuilder` struct creates the `ITunesOwner`.
#[derive(Clone, Default)]
pub struct ITunesOwnerBuilder
{
    name: Option<String>,
    email: Option<String>,
}


/// This `ITunesChannelExtension` struct contains all the items that exist for
/// the owner
/// field under 'Channel'.
#[derive(Clone)]
pub struct ITunesChannelExtension
{
    author: Option<String>,
    block: Option<String>,
    categories: Option<Vec<ITunesCategory>>,
    image: Option<String>,
    explicit: Option<String>,
    complete: Option<String>,
    new_feed_url: Option<String>,
    owner: Option<ITunesOwner>,
    subtitle: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
}


/// This `ITunesChannelExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
#[derive(Clone, Default)]
pub struct ITunesChannelExtensionBuilder
{
    author: Option<String>,
    block: Option<String>,
    categories: Option<Vec<ITunesCategory>>,
    image: Option<String>,
    explicit: Option<String>,
    complete: Option<String>,
    new_feed_url: Option<String>,
    owner: Option<ITunesOwner>,
    subtitle: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
}


/// This `ITunesItemExtension` struct contains all the items that exist for the
/// owner
/// field under 'Item'.
#[derive(Clone)]
pub struct ITunesItemExtension
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
