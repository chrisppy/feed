// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for category by using the methods under
//! `CategoryBuilder`.

use errors;
use rss::{Category, CategoryBuilder};
use url::Url;

impl CategoryBuilder {
    /// Construct a new `CategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::CategoryBuilder;
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
    /// use feed::rss::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.category("Podcast");
    /// ```
    pub fn category(&mut self, category: &str) -> &mut CategoryBuilder {
        self.category = category.to_owned();
        self
    }


    /// Set the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::CategoryBuilder;
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
    /// use feed::rss::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///         .category("Title")
    ///         .domain(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Category {
        if self.category.is_empty() {
            panic!(errors::empty_string_error("Category category"));
        }

        let mut domain = None;
        let domain_option = self.domain.clone();
        if domain_option.is_some() {
            let dom = domain_option.clone().unwrap();
            let url = Url::parse(dom.as_str())
                .expect(errors::url_parse_error(dom.as_str()).as_str());
            domain = Some(url);
        }

        Category {
            category: self.category.clone(),
            domain: domain,
        }
    }
}
