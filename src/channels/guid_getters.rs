// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under guid can be retrieved by using the methods under `Guid`.


use channels::GuidGetters;
use rss::Guid;


impl GuidGetters for Guid
{
    /// Get the permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{GuidBuilder, GuidGetters};
    ///
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use feed::channels::{GuidBuilder, GuidGetters};
    ///
    /// let permalink = true;
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(Some(permalink))
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(permalink, guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use feed::channels::{GuidBuilder, GuidGetters};
    ///
    /// let permalink = false;
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(Some(permalink))
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(permalink, guid.is_permalink());
    /// ```
    fn is_permalink(&self) -> bool
    {
        self.is_permalink
    }


    /// Get the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{GuidBuilder, GuidGetters};
    ///
    /// let guid = "9DE46946-2F90-4D5D-9047-7E9165C16E7C";
    /// let guid_obj = GuidBuilder::new()
    ///     .value(guid)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(guid.to_owned(), guid_obj.value());
    /// ```
    fn value(&self) -> String
    {
        self.value.clone()
    }
}
