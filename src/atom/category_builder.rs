// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for category by using the methods under `AtomCategoryBuilder`.

use atom::{AtomCategory, AtomCategoryBuilder};
use errors;
use url::Url;

impl AtomCategoryBuilder {
    /// Construct a new `AtomCategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// let category_builder = AtomCategoryBuilder::new();
    /// ```
    pub fn new() -> AtomCategoryBuilder {
        AtomCategoryBuilder::default()
    }


    /// Set the term that exists under `AtomCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// fn main() {
    ///     let mut category_builder = AtomCategoryBuilder::new();
    ///     category_builder.term("Rust");
    /// }
    /// ```
    pub fn term(&mut self, term: &str) -> &mut AtomCategoryBuilder {
        self.term = term.to_owned();
        self
    }


    /// Set the scheme that exists under `AtomCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// fn main() {
    ///     let mut category_builder = AtomCategoryBuilder::new();
    ///     category_builder.scheme(Some("http://www.example.com".to_owned()));
    /// }
    /// ```
    pub fn scheme(&mut self, scheme: Option<String>) -> &mut AtomCategoryBuilder {
        self.scheme = scheme;
        self
    }


    /// Set the label that exists under `AtomCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// fn main() {
    ///     let label = "General";
    ///     let mut category_builder = AtomCategoryBuilder::new();
    ///     category_builder.label(Some(label.to_owned()));
    /// }
    /// ```
    pub fn label(&mut self, label: Option<String>) -> &mut AtomCategoryBuilder {
        self.label = label;
        self
    }


    /// Construct the `AtomCategory` from the `AtomCategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// fn main() {
    ///     let category = AtomCategoryBuilder::new()
    ///         .term("Rust")
    ///         .scheme(Some("http://www.example.com".to_owned()))
    ///         .label(Some("General".to_owned()))
    ///         .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> AtomCategory {
        if self.term.is_empty() {
            panic!(errors::empty_string_error("Atom Category term"));
        }

        let mut scheme_option: Option<Url> = None;
        if self.scheme.is_some() {
            let scheme = self.scheme.clone().unwrap();
            let url = Url::parse(scheme.as_str())
                .expect(errors::url_parse_error(scheme.as_str()).as_str());
            scheme_option = Some(url);
        }
        AtomCategory {
            term: self.term.clone(),
            scheme: scheme_option,
            label: self.label.clone(),
        }
    }
}
