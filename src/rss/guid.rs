// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

//! The fields under guid can be retrieved by using the methods under `Guid`
//! and the fields can be set for guid by using the methods under `GuidBuilder`.

/// This `Guid` struct contains all the items that exist for the guid field under 'Item'.
#[derive(Clone)]
pub struct Guid {
    permalink: bool,
    guid: String,
}


impl Guid {
    /// Get the permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///     .permalink(None)
    ///     .finalize();
    /// assert!(guid.permalink());
    /// ```
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let permalink = true;
    /// let guid = GuidBuilder::new()
    ///     .permalink(Some(permalink))
    ///     .finalize();
    /// assert_eq!(permalink, guid.permalink());
    /// ```
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let permalink = false;
    /// let guid = GuidBuilder::new()
    ///     .permalink(Some(permalink))
    ///     .finalize();
    /// assert_eq!(permalink, guid.permalink());
    /// ```
    pub fn permalink(&self) -> bool {
        self.permalink.clone()
    }


    /// Get the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let guid = "9DE46946-2F90-4D5D-9047-7E9165C16E7C";
    /// let guid_obj = GuidBuilder::new()
    ///     .guid(guid)
    ///     .finalize();
    /// assert_eq!(guid.to_owned(), guid_obj.guid());
    /// ```
    pub fn guid(&self) -> String {
        self.guid.clone()
    }
}


/// This `GuidBuilder` struct creates the `Guid`.
#[derive(Default)]
pub struct GuidBuilder {
    permalink: bool,
    guid: String,
}


impl GuidBuilder {
    /// Construct a new `GuidBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
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
    /// use feed::rss::guid::GuidBuilder;
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
    /// use feed::rss::guid::GuidBuilder;
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
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///         .guid("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///         .permalink(Some(true))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Guid {
        Guid {
            permalink: self.permalink.clone(),
            guid: self.guid.clone(),
        }
    }
}
