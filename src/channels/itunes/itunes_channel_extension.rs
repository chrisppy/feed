// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under itunes category can be retrieved by using the methods under
//! `ITunesChannelExtension`.


use channels::itunes::{ITunesCategory, ITunesChannelExtension, ITunesOwner};


impl ITunesChannelExtension
{
    ///
    pub fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    ///
    pub fn block(&self) -> Option<String>
    {
        self.block.clone()
    }


    ///
    pub fn categories(&self) -> Option<Vec<ITunesCategory>>
    {
        self.categories.clone()
    }


    ///
    pub fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    ///
    pub fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    ///
    pub fn complete(&self) -> Option<String>
    {
        self.complete.clone()
    }


    ///
    pub fn new_feed_url(&self) -> Option<String>
    {
        self.new_feed_url.clone()
    }


    ///
    pub fn owner(&self) -> Option<ITunesOwner>
    {
        self.owner.clone()
    }


    ///
    pub fn subtitle(&self) -> Option<String>
    {
        self.subtitle.clone()
    }


    ///
    pub fn summary(&self) -> Option<String>
    {
        self.summary.clone()
    }


    ///
    pub fn keywords(&self) -> Option<String>
    {
        self.keywords.clone()
    }
}
