// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for category by using the methods under
//! `CategoryBuilder`.


use channels::{Category, CategoryBuilder};
use utils::string_utils;


impl CategoryBuilder {
    /// Construct a new `CategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let category_builder = CategoryBuilder::new();
    /// ```
    pub fn new() -> CategoryBuilder {
        CategoryBuilder::default()
    }


    /// Set the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.name("Podcast");
    /// ```
    pub fn name(&mut self, name: &str) -> &mut CategoryBuilder {
        self.name = name.to_owned();
        self
    }


    /// Set the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.domain(Some("http://www.example.com".to_owned()));
    /// ```
    pub fn domain(&mut self, domain: Option<String>) -> &mut CategoryBuilder {
        self.domain = domain;
        self
    }


    /// Construct the `Category` from the `CategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///         .name("Title")
    ///         .domain(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Category {
        let domain = if self.domain.is_none() {
            None
        } else {
            let d = self.domain.clone().unwrap();
            let url = string_utils::str_to_url(d.as_str(), "Category Domain");
            Some(url)
        };

        Category {
            name: self.name.clone(),
            domain: domain,
        }
    }
}
