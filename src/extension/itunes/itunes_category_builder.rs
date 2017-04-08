// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for itunes category by using the methods under
//! `ITunesCategoryBuilder`.


use extension::itunes::ITunesCategoryBuilder;
use rss::extension::itunes::ITunesCategory;


impl ITunesCategoryBuilder
{
    /// Construct a new `ITunesCategoryBuilder` and return default values.
    pub fn new() -> ITunesCategoryBuilder
    {
        ITunesCategoryBuilder::default()
    }


    /// Set the text that exists uner `ITunesCategory`.
    pub fn text(&mut self, text: &str) -> &mut ITunesCategoryBuilder
    {
        self.text = text.to_owned();
        self
    }


    /// Set the optional subcategory that exists uner `ITunesCategory`.
    pub fn subcategory(&mut self, subcategory: Option<Box<ITunesCategory>>) -> &mut ITunesCategoryBuilder
    {
        self.subcategory = subcategory;
        self
    }


    /// Construct the `ITunesCategory` from the `ITunesCategoryBuilder`.
    pub fn finalize(&self) -> Result<ITunesCategory, String>
    {
        Ok(ITunesCategory {
               text: self.text.clone(),
               subcategory: self.subcategory.clone(),
           })
    }
}
