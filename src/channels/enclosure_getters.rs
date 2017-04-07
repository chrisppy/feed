// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under enclosure can be retrieved by using the methods under
//! `Enclosure`.


use channels::EnclosureGetters;
use rss::Enclosure;


impl EnclosureGetters for Enclosure
{
    /// Get the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{EnclosureBuilder, EnclosureGetters};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(url.to_owned(), enclosure.url())
    /// ```
    fn url(&self) -> String
    {
        self.url.clone()
    }


    /// Get the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{EnclosureBuilder, EnclosureGetters};
    ///
    /// let length: i64 = 70772893;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .length(length)
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(length.to_string(), enclosure.length())
    /// ```
    fn length(&self) -> String
    {
        self.length.clone()
    }


    /// Get the enclosure type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{EnclosureBuilder, EnclosureGetters};
    ///
    /// let enclosure_type = "audio/ogg";
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .mime_type(enclosure_type)
    ///     .finalize()
    ///     .unwrap();
    /// assert_eq!(enclosure_type.to_owned(), enclosure.mime_type())
    /// ```
    fn mime_type(&self) -> String
    {
        self.mime_type.clone()
    }
}
