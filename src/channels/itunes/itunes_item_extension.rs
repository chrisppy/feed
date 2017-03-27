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


use channels::itunes::ITunesItemExtension;


impl ITunesItemExtension
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
    pub fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    ///
    pub fn duration(&self) -> Option<String>
    {
        self.duration.clone()
    }


    ///
    pub fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    ///
    pub fn closed_captioned(&self) -> Option<String>
    {
        self.closed_captioned.clone()
    }


    ///
    pub fn order(&self) -> Option<String>
    {
        self.order.clone()
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
