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


use extension::itunes::ITunesCategoryGetters;
use rss::extension::itunes::ITunesCategory;

impl ITunesCategoryGetters for ITunesCategory
{
    /// Get the text that exists under `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesCategoryBuilder, ITunesCategoryGetters};
    ///
    /// let text = "text";
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text(text)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(text.to_owned(), category.text())
    /// ```
    fn text(&self) -> String
    {
        self.text.clone()
    }


    /// Get the optional subcategory that exists under `ITunesCategory`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesCategoryBuilder, ITunesCategoryGetters};
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();;
    ///
    /// assert!(category.subcategory().is_some());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesCategoryBuilder, ITunesCategoryGetters};
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(None)
    ///     .finalize()
    ///     .unwrap();;
    ///
    /// assert!(category.subcategory().is_none());
    /// ```
    fn subcategory(&self) -> Option<Box<ITunesCategory>>
    {
        self.subcategory.clone()
    }
}
