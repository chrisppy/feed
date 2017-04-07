// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under itunes category can be retrieved by using the methods under
//! `ITunesCategory`.


use channels::itunes::ITunesCategory;


impl ITunesCategory
{
    ///
    pub fn text(&self) -> String
    {
        self.text.clone()
    }


    ///
    pub fn subcategory(&self) -> Option<Box<ITunesCategory>>
    {
        self.subcategory.clone()
    }
}
