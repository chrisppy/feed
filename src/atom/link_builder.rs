// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for link by using the methods under `LinkBuilder`.

use atom::{Link, LinkBuilder};
use errors;
use url::Url;

impl LinkBuilder {
    /// Construct a new `LinkBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::LinkBuilder;
    ///
    /// let link_builder = LinkBuilder::new();
    /// ```
    pub fn new() -> LinkBuilder {
        LinkBuilder::default()
    }


    /// Set the href that exists under `Link`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let mut link_builder = LinkBuilder::new();
    ///     link_builder.href("http://www.newrustacean.com/feed.xml");
    /// }
    /// ```
    pub fn href(&mut self, href: &str) -> &mut LinkBuilder {
        self.href = href.to_owned();
        self
    }


    /// Set the rel that exists under `Link`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::LinkBuilder;
    ///
    /// let mut link_builder = LinkBuilder::new();
    /// link_builder.rel(Some("self".to_owned()));
    /// ```
    pub fn rel(&mut self, rel: Option<String>) -> &mut LinkBuilder {
        if rel.is_some() {
            let rel_value = rel.clone().unwrap();
            match rel_value.as_str() {
                "alternate" => (),
                "enclosure" => (),
                "related" => (),
                "self" => (),
                "via" => (),
                _ => {
                    let rel_str = rel_value.as_str();
                    Url::parse(rel_str).expect(errors::url_parse_error(rel_str).as_str());
                }
            };
        }
        self.rel = rel;
        self
    }


    /// Set the type that exists under `Link`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::LinkBuilder;
    ///
    /// let mut link_builder = LinkBuilder::new();
    /// link_builder.link_type(Some("application/rss+xml".to_owned()));
    /// ```
    pub fn link_type(&mut self, link_type: Option<String>) -> &mut LinkBuilder {
        self.link_type = link_type;
        self
    }


    /// Set the href_lang that exists under `Link`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::LinkBuilder;
    ///
    /// let mut link_builder = LinkBuilder::new();
    /// link_builder.href_lang(Some("en".to_owned()));
    /// ```
    pub fn href_lang(&mut self, href_lang: Option<String>) -> &mut LinkBuilder {
        self.href_lang = href_lang;
        self
    }


    /// Set the title that exists under `Link`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::LinkBuilder;
    ///
    /// let mut link_builder = LinkBuilder::new();
    /// link_builder.title(Some("New Rustacean".to_owned()));
    /// ```
    pub fn title(&mut self, title: Option<String>) -> &mut LinkBuilder {
        self.title = title;
        self
    }


    /// Set the length that exists under `Link`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::LinkBuilder;
    ///
    /// let mut link_builder = LinkBuilder::new();
    /// link_builder.length(Some(64));
    /// ```
    pub fn length(&mut self, length: Option<i64>) -> &mut LinkBuilder {
        if length.is_some() {
            let len = length.unwrap();
            if len < 0 {
                panic!(errors::negative_error("atom link length", len));
            }
        }
        self.length = length;
        self
    }


    /// Construct the `Person` from the `PersonBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::LinkBuilder;
    ///
    /// fn main() {
    ///     let link = LinkBuilder::new()
    ///         .href("http://www.newrustacean.com/feed.xml")
    ///         .rel(Some("self".to_owned()))
    ///         .link_type(Some("application/rss+xml".to_owned()))
    ///         .href_lang(Some("en".to_owned()))
    ///         .title(Some("New Rustacean".to_owned()))
    ///         .length(Some(64))
    ///         .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> Link {
        if self.href.is_empty() {
            panic!(errors::empty_string_error("Atom Link href"));
        }

        let href = self.href.clone();
        Link {
            href: Url::parse(href.as_str()).expect(errors::url_parse_error(href.as_str()).as_str()),
            rel: self.rel.clone(),
            link_type: self.link_type.clone(),
            href_lang: self.href_lang.clone(),
            title: self.title.clone(),
            length: self.length.clone(),
        }
    }
}
