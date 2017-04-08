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
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let category_builder = ITunesCategoryBuilder::new();
    /// ```
    pub fn new() -> ITunesCategoryBuilder
    {
        ITunesCategoryBuilder::default()
    }


    /// Set the text that exists uner `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let mut category_builder = ITunesCategoryBuilder::new();
    /// category_builder.text("text");
    /// ```
    pub fn text(&mut self, text: &str) -> &mut ITunesCategoryBuilder
    {
        self.text = text.to_owned();
        self
    }


    /// Set the optional subcategory that exists uner `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut category_builder = ITunesCategoryBuilder::new();
    /// category_builder.subcategory(Some(Box::new(subcategory)));
    /// ```
    pub fn subcategory(&mut self, subcategory: Option<Box<ITunesCategory>>) -> &mut ITunesCategoryBuilder
    {
        self.subcategory = subcategory;
        self
    }


    /// Construct the `ITunesCategory` from the `ITunesCategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();;
    /// ```
    pub fn finalize(&self) -> Result<ITunesCategory, String>
    {
        Ok(ITunesCategory {
               text: self.text.clone(),
               subcategory: self.subcategory.clone(),
           })
    }
}
