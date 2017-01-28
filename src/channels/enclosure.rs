// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under enclosure can be retrieved by using the methods under `Enclosure`.


use channels::Enclosure;
use mime::Mime;
use url::Url;


impl Enclosure
{
    /// Get the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .mime_type("audio/ogg")
    ///     .finalize();
    /// assert_eq!(url.to_owned(), enclosure.url().into_string())
    /// ```
    pub fn url(&self) -> Url
    {
        self.url.clone()
    }


    /// Get the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::EnclosureBuilder;
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
    ///     .finalize();
    /// assert_eq!(length, enclosure.length())
    /// ```
    pub fn length(&self) -> i64
    {
        self.length
    }


    /// Get the enclosure type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::EnclosureBuilder;
    ///
    /// let enclosure_type = "audio/ogg";
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .mime_type(enclosure_type)
    ///     .finalize();
    /// assert_eq!(enclosure_type.to_owned(), enclosure.mime_type().to_string())
    /// ```
    pub fn mime_type(&self) -> Mime
    {
        self.mime_type.clone()
    }
}
