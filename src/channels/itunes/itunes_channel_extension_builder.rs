// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for itunes category by using the methods under
//! `ITunesChannelExtensionBuilder`.


use channels::itunes::{ITunesCategory, ITunesChannelExtension, ITunesChannelExtensionBuilder, ITunesOwner};


impl ITunesChannelExtensionBuilder
{
    /// Construct a new `ITunesChannelExtension` and return default values.
    pub fn new() -> ITunesChannelExtensionBuilder
    {
        ITunesChannelExtensionBuilder::default()
    }


    /// Set the optional author that exists uner `ITunesChannelExtension`.
    pub fn author(&mut self, author: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional block that exists uner `ITunesChannelExtension`.
    pub fn block(&mut self, block: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.block = block;
        self
    }


    /// Set the categories that exists uner `ITunesChannelExtension`.
    pub fn categories(&mut self, categories: Option<Vec<ITunesCategory>>) -> &mut ITunesChannelExtensionBuilder
    {
        self.categories = categories;
        self
    }


    /// Set the optional image that exists uner `ITunesChannelExtension`.
    pub fn image(&mut self, image: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.image = image;
        self
    }


    /// Set the optional explicit that exists uner `ITunesChannelExtension`.
    pub fn explicit(&mut self, explicit: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    /// Set the optional complete that exists uner `ITunesChannelExtension`.
    pub fn complete(&mut self, complete: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.complete = complete;
        self
    }


    /// Set the optional new_feed_url that exists uner `ITunesChannelExtension`.
    pub fn new_feed_url(&mut self, new_feed_url: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.new_feed_url = new_feed_url;
        self
    }


    /// Set the optional owner that exists uner `ITunesChannelExtension`.
    pub fn owner(&mut self, owner: Option<ITunesOwner>) -> &mut ITunesChannelExtensionBuilder
    {
        self.owner = owner;
        self
    }


    /// Set the optional subtitle that exists uner `ITunesChannelExtension`.
    pub fn subtitle(&mut self, subtitle: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    /// Set the optional summary that exists uner `ITunesChannelExtension`.
    pub fn summary(&mut self, summary: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.summary = summary;
        self
    }


    /// Set the optional keywords that exists uner `ITunesChannelExtension`.
    pub fn keywords(&mut self, keywords: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesChannelExtension` from the `ITunesChannelExtensionBuilder`.
    pub fn finalize(&self) -> Result<ITunesChannelExtension, String>
    {
        let categories: Vec<ITunesCategory> = match self.categories.clone()
        {
            Some(val) => val,
            None => Vec::new(),
        };

        Ok(ITunesChannelExtension {
               author: self.author.clone(),
               block: self.block.clone(),
               categories: categories,
               image: self.image.clone(),
               explicit: self.explicit.clone(),
               complete: self.complete.clone(),
               new_feed_url: self.new_feed_url.clone(),
               owner: self.owner.clone(),
               subtitle: self.subtitle.clone(),
               summary: self.summary.clone(),
               keywords: self.keywords.clone(),
           })
    }
}
