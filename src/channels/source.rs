// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under source can be retrieved by using the methods under `Source`.


use channel::Source;


impl Source {
    /// Get the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channel::SourceBuilder;
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
    /// use feed::channel::SourceBuilder;
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
