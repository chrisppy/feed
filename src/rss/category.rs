// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under category can be retrieved by using the methods under
//! `Category`.

use rss::Category;
use url::Url;

impl Category {
    /// Get the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::CategoryBuilder;
    ///
    /// let category = "podcast";
    /// let category_obj = CategoryBuilder::new()
    ///     .category(category)
    ///     .finalize();
    /// assert_eq!(category.to_owned(), category_obj.category());
    /// ```
    pub fn category(&self) -> String {
        self.category.clone()
    }


    /// Get the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::CategoryBuilder;
    ///
    /// let domain_string = "http://jupiterbroadcasting.com/".to_owned();
    /// let category = CategoryBuilder::new()
    ///     .category("Title")
    ///     .domain(Some(domain_string.clone()))
    ///     .finalize();
    /// let domain_option = category.domain();
    /// assert!(domain_option.is_some());
    /// let domain = domain_option.unwrap();
    /// assert_eq!(domain_string.clone(), domain.into_string());
    /// ```
    ///
    /// ```
    /// use feed::rss::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///     .category("Title")
    ///     .domain(None)
    ///     .finalize();
    /// let domain_option = category.domain();
    /// assert!(domain_option.is_none());
    /// ```
    pub fn domain(&self) -> Option<Url> {
        self.domain.clone()
    }
}
