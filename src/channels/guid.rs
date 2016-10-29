// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under guid can be retrieved by using the methods under `Guid`.


use channels::Guid;


impl Guid {
    /// Get the permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///     .permalink(None)
    ///     .finalize();
    /// assert!(guid.permalink());
    /// ```
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
    ///
    /// let permalink = true;
    /// let guid = GuidBuilder::new()
    ///     .permalink(Some(permalink))
    ///     .finalize();
    /// assert_eq!(permalink, guid.permalink());
    /// ```
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
    ///
    /// let permalink = false;
    /// let guid = GuidBuilder::new()
    ///     .permalink(Some(permalink))
    ///     .finalize();
    /// assert_eq!(permalink, guid.permalink());
    /// ```
    pub fn permalink(&self) -> bool {
        self.permalink
    }


    /// Get the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
    ///
    /// let guid = "9DE46946-2F90-4D5D-9047-7E9165C16E7C";
    /// let guid_obj = GuidBuilder::new()
    ///     .value(guid)
    ///     .finalize();
    /// assert_eq!(guid.to_owned(), guid_obj.value());
    /// ```
    pub fn value(&self) -> String {
        self.value.clone()
    }
}
