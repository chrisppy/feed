// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for category by using the methods under
//! `CategoryBuilder`.


use channels::{Category, CategoryBuilder};
use utils::string_utils;


impl CategoryBuilder
{
    /// Construct a new `CategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let category_builder = CategoryBuilder::new();
    /// ```
    pub fn new() -> CategoryBuilder
    {
        CategoryBuilder::default()
    }


    /// Set the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.name("Podcast");
    /// ```
    pub fn name(&mut self, name: &str) -> &mut CategoryBuilder
    {
        self.name = name.to_owned();
        self
    }


    /// Set the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.domain(Some("http://www.example.com".to_owned()));
    /// ```
    pub fn domain(&mut self, domain: Option<String>) -> &mut CategoryBuilder
    {
        self.domain = domain;
        self
    }


    /// Construct the `Category` from the `CategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///         .name("Title")
    ///         .domain(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Result<Category, String>
    {
        let domain = match self.domain.clone()
        {
            Some(val) => Some(string_utils::str_to_url(val.as_str())?),
            None => None,
        };

        Ok(Category {
               name: self.name.clone(),
               domain: domain,
           })
    }
}
