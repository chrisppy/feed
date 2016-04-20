// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for guid by using the methods under `GuidBuilder`.

use rss::{Guid, GuidBuilder};

impl GuidBuilder {
    /// Construct a new `GuidBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::GuidBuilder;
    ///
    /// let guid_builder = GuidBuilder::new();
    /// ```
    pub fn new() -> GuidBuilder {
        GuidBuilder::default()
    }


    /// Set the optional permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.permalink(Some(false));
    /// ```
    pub fn permalink(&mut self, permalink: Option<bool>) -> &mut GuidBuilder {
        if permalink.is_some() {
            self.permalink = permalink.unwrap();
        } else {
            self.permalink = true;
        }
        self
    }


    /// Set the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.guid("9DE46946-2F90-4D5D-9047-7E9165C16E7C");
    /// ```
    pub fn guid(&mut self, guid: &str) -> &mut GuidBuilder {
        self.guid = guid.to_owned();
        self
    }


    /// Construct the `Guid` from the `GuidBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///         .guid("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///         .permalink(Some(true))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Guid {
        Guid {
            permalink: self.permalink,
            guid: self.guid.clone(),
        }
    }
}
