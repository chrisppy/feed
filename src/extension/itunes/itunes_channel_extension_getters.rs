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


use extension::itunes::ITunesChannelExtensionGetters;
use rss::extension::itunes::{ITunesCategory, ITunesChannelExtension, ITunesOwner};


impl ITunesChannelExtensionGetters for ITunesChannelExtension
{
    /// Get the optional author that exists under `ITunesChannelExtension`.
    fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    /// Get the optional block that exists under `ITunesChannelExtension`.
    fn block(&self) -> Option<String>
    {
        self.block.clone()
    }


    /// Get the categories that exists under `ITunesChannelExtension`.
    fn categories(&self) -> Vec<ITunesCategory>
    {
        self.categories.clone()
    }


    /// Get the optional image that exists under `ITunesChannelExtension`.
    fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    /// Get the optional explicit that exists under `ITunesChannelExtension`.
    fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    /// Get the optional complete that exists under `ITunesChannelExtension`.
    fn complete(&self) -> Option<String>
    {
        self.complete.clone()
    }


    /// Get the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    fn new_feed_url(&self) -> Option<String>
    {
        self.new_feed_url.clone()
    }


    /// Get the optional owner that exists under `ITunesChannelExtension`.
    fn owner(&self) -> Option<ITunesOwner>
    {
        self.owner.clone()
    }


    /// Get the optional subtitle that exists under `ITunesChannelExtension`.
    fn subtitle(&self) -> Option<String>
    {
        self.subtitle.clone()
    }


    /// Get the optional summary that exists under `ITunesChannelExtension`.
    fn summary(&self) -> Option<String>
    {
        self.summary.clone()
    }


    /// Get the optional keywords that exists under `ITunesChannelExtension`.
    fn keywords(&self) -> Option<String>
    {
        self.keywords.clone()
    }
}
