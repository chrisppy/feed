// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for guid by using the methods under `GuidBuilder`.


use GuidBuilder;
use rss::Guid;


impl GuidBuilder
{
    /// Construct a new `GuidBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::GuidBuilder;
    ///
    /// let guid_builder = GuidBuilder::new();
    /// ```
    pub fn new() -> GuidBuilder
    {
        GuidBuilder::default()
    }


    /// Set the optional permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.is_permalink(Some(false));
    /// ```
    pub fn is_permalink(&mut self, is_permalink: Option<bool>) -> &mut GuidBuilder
    {
        self.is_permalink = is_permalink;
        self
    }


    /// Set the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.value("9DE46946-2F90-4D5D-9047-7E9165C16E7C");
    /// ```
    pub fn value(&mut self, value: &str) -> &mut GuidBuilder
    {
        self.value = value.to_owned();
        self
    }


    /// Construct the `Guid` from the `GuidBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///         .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///         .is_permalink(Some(true))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Result<Guid, String>
    {
        let is_permalink = match self.is_permalink
        {
            Some(val) => val,
            None => true,
        };

        Ok(Guid {
               is_permalink: is_permalink,
               value: self.value.clone(),
           })
    }
}
