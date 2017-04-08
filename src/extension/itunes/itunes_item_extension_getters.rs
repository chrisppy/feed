// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under itunes category can be retrieved by using the methods under
//! `ITunesItemExtension`.


use extension::itunes::ITunesItemExtensionGetters;
use rss::extension::itunes::ITunesItemExtension;


impl ITunesItemExtensionGetters for ITunesItemExtension
{
    /// Get the optional author that exists under `ITunesItemExtension`.
    fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    /// Get the optional block that exists under `ITunesItemExtension`.
    fn block(&self) -> Option<String>
    {
        self.block.clone()
    }


    /// Get the optional image that exists under `ITunesItemExtension`.
    fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    /// Get the optional duration that exists under `ITunesItemExtension`.
    fn duration(&self) -> Option<String>
    {
        self.duration.clone()
    }


    /// Get the optional explicit that exists under `ITunesItemExtension`.
    fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    /// Get the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    fn closed_captioned(&self) -> Option<String>
    {
        self.closed_captioned.clone()
    }


    /// Get the optional order that exists under `ITunesItemExtension`.
    fn order(&self) -> Option<String>
    {
        self.order.clone()
    }


    /// Get the optional subtitle that exists under `ITunesItemExtension`.
    fn subtitle(&self) -> Option<String>
    {
        self.subtitle.clone()
    }


    /// Get the optional summary that exists under `ITunesItemExtension`.
    fn summary(&self) -> Option<String>
    {
        self.summary.clone()
    }


    /// Get the optional keywords that exists under `ITunesItemExtension`.
    fn keywords(&self) -> Option<String>
    {
        self.keywords.clone()
    }
}
