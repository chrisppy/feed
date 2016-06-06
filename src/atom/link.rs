// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under link can be retrieved by using the methods under `link`.

use atom::Link;
use url::Url;

impl Link {
    /// Get the href that exists under `link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let href = "http://www.newrustacean.com/feed.xml";
    ///     let link = LinkBuilder::new()
    ///         .href(href.clone())
    ///         .finalize();
    ///     assert_eq!(href.clone().to_owned(), link.href().into_string());
    /// }
    /// ```
    pub fn href(&self) -> Url {
        self.href.clone()
    }


    /// Get the rel that exists under `link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let rel = "self";
    ///     let link = LinkBuilder::new()
    ///         .href("http://www.newrustacean.com/feed.xml")
    ///         .rel(Some(rel.to_owned()))
    ///         .finalize();
    ///     assert_eq!(rel, link.rel().unwrap());
    /// }
    /// ```
    pub fn rel(&self) -> Option<String> {
        self.rel.clone()
    }


    /// Get the link_type that exists under `link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let link_type = "application/rss+xml";
    ///     let link = LinkBuilder::new()
    ///         .href("http://www.newrustacean.com/feed.xml")
    ///         .link_type(Some(link_type.to_owned()))
    ///         .finalize();
    ///     assert_eq!(link_type, link.link_type().unwrap());
    /// }
    /// ```
    pub fn link_type(&self) -> Option<String> {
        self.link_type.clone()
    }


    /// Get the href_lang that exists under `link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let href_lang = "en";
    ///     let link = LinkBuilder::new()
    ///         .href("http://www.newrustacean.com/feed.xml")
    ///         .href_lang(Some(href_lang.to_owned()))
    ///         .finalize();
    ///     assert_eq!(href_lang, link.href_lang().unwrap());
    /// }
    /// ```
    pub fn href_lang(&self) -> Option<String> {
        self.href_lang.clone()
    }


    /// Get the title that exists under `link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let title = "New Rustacean";
    ///     let link = LinkBuilder::new()
    ///         .href("http://www.newrustacean.com/feed.xml")
    ///         .title(Some(title.to_owned()))
    ///         .finalize();
    ///     assert_eq!(title, link.title().unwrap());
    /// }
    /// ```
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }


    /// Get the length that exists under `link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let length: i64 = 64;
    ///     let link = LinkBuilder::new()
    ///         .href("http://www.newrustacean.com/feed.xml")
    ///         .length(Some(length))
    ///         .finalize();
    ///     assert_eq!(length, link.length().unwrap());
    /// }
    /// ```
    pub fn length(&self) -> Option<i64> {
        self.length.clone()
    }
}
