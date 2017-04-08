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


use channels::itunes::ITunesItemExtensionBuilder;
use rss::extension::itunes::ITunesItemExtension;


impl ITunesItemExtensionBuilder
{
    /// Construct a new `ITunesItemExtensionBuilder` and return default values.
    pub fn new() -> ITunesItemExtensionBuilder
    {
        ITunesItemExtensionBuilder::default()
    }


    /// Set the optional author that exists uner `ITunesItemExtension`.
    pub fn author(&mut self, author: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional block that exists uner `ITunesItemExtension`.
    pub fn block(&mut self, block: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.block = block;
        self
    }


    /// Set the optional image that exists uner `ITunesItemExtension`.
    pub fn image(&mut self, image: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.image = image;
        self
    }


    /// Set the optional duration that exists uner `ITunesItemExtension`.
    pub fn duration(&mut self, duration: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.duration = duration;
        self
    }


    /// Set the optional explicit that exists uner `ITunesItemExtension`.
    pub fn explicit(&mut self, explicit: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    /// Set the optional closed_captioned that exists uner
    /// `ITunesItemExtension`.
    pub fn closed_captioned(&mut self, closed_captioned: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.closed_captioned = closed_captioned;
        self
    }


    /// Set the optional order that exists uner `ITunesItemExtension`.
    pub fn order(&mut self, order: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.order = order;
        self
    }


    /// Set the optional subtitle that exists uner `ITunesItemExtension`.
    pub fn subtitle(&mut self, subtitle: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    /// Set the optional summary that exists uner `ITunesItemExtension`.
    pub fn summary(&mut self, summary: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.summary = summary;
        self
    }


    /// Set the optional keywords that exists uner `ITunesItemExtension`.
    pub fn keywords(&mut self, keywords: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesItemExtension` from the
    /// `ITunesItemExtensionBuilder`.
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
