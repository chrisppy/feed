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
    ///
    pub fn new() -> ITunesChannelExtensionBuilder
    {
        ITunesChannelExtensionBuilder::default()
    }


    ///
    pub fn author(&mut self, author: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.author = author;
        self
    }


    ///
    pub fn block(&mut self, block: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.block = block;
        self
    }


    ///
    pub fn categories(&mut self, categories: Option<Vec<ITunesCategory>>) -> &mut ITunesChannelExtensionBuilder
    {
        self.categories = categories;
        self
    }


    ///
    pub fn image(&mut self, image: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.image = image;
        self
    }


    ///
    pub fn explicit(&mut self, explicit: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    ///
    pub fn complete(&mut self, complete: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.complete = complete;
        self
    }


    ///
    pub fn new_feed_url(&mut self, new_feed_url: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.new_feed_url = new_feed_url;
        self
    }


    ///
    pub fn owner(&mut self, owner: Option<ITunesOwner>) -> &mut ITunesChannelExtensionBuilder
    {
        self.owner = owner;
        self
    }


    ///
    pub fn subtitle(&mut self, subtitle: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    ///
    pub fn summary(&mut self, summary: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.summary = summary;
        self
    }


    ///
    pub fn keywords(&mut self, keywords: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    ///
    pub fn finalize(&self) -> Result<ITunesChannelExtension, String>
    {
        Ok(ITunesChannelExtension {
               author: self.author.clone(),
               block: self.block.clone(),
               categories: self.categories.clone(),
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
