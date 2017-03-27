// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for itunes category by using the methods under
//! `ITunesItemExtensionBuilder`.


use channels::itunes::{ITunesItemExtension, ITunesItemExtensionBuilder};


impl ITunesItemExtensionBuilder
{
    ///
    pub fn new() -> ITunesItemExtensionBuilder
    {
        ITunesItemExtensionBuilder::default()
    }


    ///
    pub fn author(&mut self, author: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.author = author;
        self
    }


    ///
    pub fn block(&mut self, block: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.block = block;
        self
    }


    ///
    pub fn image(&mut self, image: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.image = image;
        self
    }


    ///
    pub fn duration(&mut self, duration: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.duration = duration;
        self
    }


    ///
    pub fn explicit(&mut self, explicit: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    ///
    pub fn closed_captioned(&mut self, closed_captioned: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.closed_captioned = closed_captioned;
        self
    }


    ///
    pub fn order(&mut self, order: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.order = order;
        self
    }


    ///
    pub fn subtitle(&mut self, subtitle: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    ///
    pub fn summary(&mut self, summary: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.summary = summary;
        self
    }


    ///
    pub fn keywords(&mut self, keywords: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    ///
    pub fn finalize(&self) -> Result<ITunesItemExtension, String>
    {
        Ok(ITunesItemExtension {
               author: self.author.clone(),
               block: self.block.clone(),
               image: self.image.clone(),
               duration: self.duration.clone(),
               explicit: self.explicit.clone(),
               closed_captioned: self.closed_captioned.clone(),
               order: self.order.clone(),
               subtitle: self.subtitle.clone(),
               summary: self.summary.clone(),
               keywords: self.keywords.clone(),
           })
    }
}
