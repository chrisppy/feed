// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for source by using the methods under `SourceBuilder`.

use errors;
use rss::{Source, SourceBuilder};
use url::Url;

impl SourceBuilder {
    /// Construct a new `SourceBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::SourceBuilder;
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
    /// use feed::rss::SourceBuilder;
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
    /// use feed::rss::SourceBuilder;
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
    /// use feed::rss::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///         .url("http://www.example.com/source")
    ///         .source("Test")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Source {
        if self.url.is_empty() {
            panic!(errors::empty_string_error("Source url"));
        }
        if self.source.is_empty() {
            panic!(errors::empty_string_error("Source source"));
        }

        let url_str = self.url.clone();
        let url = Url::parse(url_str.as_str())
            .expect(errors::url_parse_error(url_str.as_str()).as_str());

        Source {
            url: url,
            source: self.source.clone(),
        }
    }
}
