// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under source can be retrieved by using the methods under
//! `Source`.


use channels::SourceGetters;
use rss::Source;


impl SourceGetters for Source
{
    /// Get the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source = SourceBuilder::new()
    ///     .url(url)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(url.to_owned(), source.url().into_string());
    /// ```
    fn url(&self) -> String
    {
        self.url.clone()
    }


    /// Get the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::SourceBuilder;
    ///
    /// let title = "Tomalak's Realm";
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source_obj = SourceBuilder::new()
    ///     .title(Some(title.to_owned()))
    ///     .url(url)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(title.to_owned(), source_obj.title().unwrap());
    /// ```
    fn title(&self) -> Option<String>
    {
        self.title.clone()
    }
}
