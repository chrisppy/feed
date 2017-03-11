// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for source by using the methods under `SourceBuilder`.


use channels::{Source, SourceBuilder};
use utils::string_utils;


impl SourceBuilder
{
    /// Construct a new `SourceBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let source_builder = SourceBuilder::new();
    /// ```
    pub fn new() -> SourceBuilder
    {
        SourceBuilder::default()
    }


    /// Set the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.url("http://www.example.com/source");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut SourceBuilder
    {
        self.url = url.to_owned();
        self
    }


    /// Set the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.title(Some("Test".to_owned()));
    /// ```
    pub fn title(&mut self, title: Option<String>) -> &mut SourceBuilder
    {
        self.title = title;
        self
    }


    /// Construct the `Source` from the `SourceBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///         .url("http://www.example.com/source")
    ///         .title(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Result<Source, String>
    {
        let url = string_utils::str_to_url(self.url.as_str())?;

        Ok(Source {
               url: url,
               title: self.title.clone(),
           })
    }
}
