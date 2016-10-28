// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for guid by using the methods under `GuidBuilder`.


use channels::{Guid, GuidBuilder};


impl GuidBuilder {
    /// Construct a new `GuidBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
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
    /// use feed::channels::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.is_permalink(Some(false));
    /// ```
    pub fn is_permalink(&mut self, is_permalink: Option<bool>) -> &mut GuidBuilder {
        if is_permalink.is_some() {
            self.is_permalink = is_permalink.unwrap();
        } else {
            self.is_permalink = true;
        }
        self
    }


    /// Set the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.value("9DE46946-2F90-4D5D-9047-7E9165C16E7C");
    /// ```
    pub fn value(&mut self, value: &str) -> &mut GuidBuilder {
        self.value = value.to_owned();
        self
    }


    /// Construct the `Guid` from the `GuidBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///         .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///         .is_permalink(Some(true))
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Guid {
        Guid {
            is_permalink: self.is_permalink,
            value: self.value.clone(),
        }
    }
}
