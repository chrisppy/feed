// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under source can be retrieved by using the methods under `Source`
//! and the fields can be set for source by using the methods under `SourceBuilder`.

/// This `Source` struct contains all the items that exist for the source field under 'Item'.
#[derive(Clone)]
pub struct Source {
    url: String,
    source: String,
}


impl Source {
    /// Get the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    /// let source = SourceBuilder::new()
    ///     .url(url)
    ///     .finalize();
    /// assert_eq!(url.to_owned(), source.url());
    /// ```
    pub fn url(&self) -> String {
        self.url.clone()
    }


    /// Get the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let source = "Tomalak's Realm";
    /// let source_obj = SourceBuilder::new()
    ///     .source(source)
    ///     .finalize();
    /// assert_eq!(source.to_owned(), source_obj.source());
    /// ```
    pub fn source(&self) -> String {
        self.source.clone()
    }
}


/// This `SourceBuilder` struct creates the `Source`.
#[derive(Default)]
pub struct SourceBuilder {
    url: String,
    source: String,
}


impl SourceBuilder {
    /// Construct a new `SourceBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let source_builder = SourceBuilder::new();
    /// ```
    pub fn new() -> SourceBuilder {
        SourceBuilder::default()
    }


    /// Set the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.url("http://www.example.com/source");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut SourceBuilder {
        self.url = url.to_owned();
        self
    }


    /// Set the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.source("Test");
    /// ```
    pub fn source(&mut self, source: &str) -> &mut SourceBuilder {
        self.source = source.to_owned();
        self
    }


    /// Construct the `Source` from the `SourceBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///         .url("http://www.example.com/source")
    ///         .source("Test")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Source {
        Source {
            url: self.url.clone(),
            source: self.source.clone(),
        }
    }
}
