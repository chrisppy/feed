
// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under category can be retrieved by using the methods under `AtomCategory`.

use atom::AtomCategory;
use url::Url;

impl AtomCategory {
    /// Get the term that exists under `AtomCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// fn main() {
    ///     let term = "Rust";
    ///     let category = AtomCategoryBuilder::new()
    ///         .term(term)
    ///         .finalize();
    ///     assert_eq!(term.to_owned(), category.term());
    /// }
    /// ```
    pub fn term(&self) -> String {
        self.term.clone()
    }


    /// Get the scheme that exists under `AtomCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomCategoryBuilder;
    ///
    /// fn main() {
    ///     let scheme = "http://www.example.com/".to_owned();
    ///     let category = AtomCategoryBuilder::new()
    ///         .scheme(Some(scheme.clone()))
    ///         .finalize();
    ///     assert_eq!(scheme.clone(), category.scheme().unwrap().into_string());
    /// }
    /// ```
    pub fn scheme(&self) -> Option<Url> {
        self.scheme.clone()
    }


    /// Get the label that exists under `AtomCategory`.
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
    ///     let category = AtomCategoryBuilder::new()
    ///         .label(Some(label.to_owned()))
    ///         .finalize();
    ///     assert_eq!(Some(label.to_owned()), category.label());
    /// }
    /// ```
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }
}
