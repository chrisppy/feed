// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under source can be retrieved by using the methods under `Source`.


use channels::Source;
use url::Url;


impl Source {
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
    ///     .finalize();
    ///
    /// assert_eq!(url.to_owned(), source.url().into_string());
    /// ```
    pub fn url(&self) -> Url {
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
    ///     .finalize();
    /// assert_eq!(title.to_owned(), source_obj.title().unwrap());
    /// ```
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }
}
