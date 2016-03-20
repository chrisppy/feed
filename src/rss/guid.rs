// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

/*!
 * The fields under guid can be retrieved by using the methods under `Guid`
 * and the fields can be set for guid by using the methods under `GuidBuilder`.
 */

/// This `Guid` struct contains all the items that exist for the guid field under 'Item'.
#[derive(Clone)]
pub struct Guid {
    is_permalink: bool,
    guid:         String,
}


impl Guid {
    /// Get the is_permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(None)
    ///     .finalize();
    /// assert!(guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let is_permalink = true;
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(Some(is_permalink))
    ///     .finalize();
    /// assert_eq!(is_permalink, guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let is_permalink = false;
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(Some(is_permalink))
    ///     .finalize();
    /// assert_eq!(is_permalink, guid.is_permalink());
    /// ```
    pub fn is_permalink(&self) -> bool {
        self.is_permalink.clone()
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
    /// assert_eq!(guid.to_string(), guid_obj.guid());
    /// ```
    pub fn guid(&self) -> String {
        self.guid.clone()
    }
}


/// This `GuidBuilder` struct creates the `Guid`.
pub struct GuidBuilder {
    is_permalink: bool,
    guid:         String,
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
        GuidBuilder {
            is_permalink: true,
            guid:         String::new(),
        }
    }


    /// Set the optional is_permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.is_permalink(Some(false));
    /// ```
    pub fn is_permalink(&mut self, is_permalink: Option<bool>) -> &mut GuidBuilder {
        if is_permalink.is_some() {
            self.is_permalink = is_permalink.unwrap();
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
        self.guid = guid.to_string();
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
    ///         .is_permalink(Some(true))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Guid {
        Guid {
            is_permalink: self.is_permalink.clone(),
            guid:         self.guid.clone(),
        }
    }
}
