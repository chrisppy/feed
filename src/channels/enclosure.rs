// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under enclosure can be retrieved by using the methods under `Enclosure`.


use channels::Enclosure;
use url::Url;


impl Enclosure {
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
    ///     .finalize();
    /// assert_eq!(url.to_owned(), enclosure.url().into_string())
    /// ```
    pub fn url(&self) -> Url {
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
    ///     .finalize();
    /// assert_eq!(length, enclosure.length())
    /// ```
    pub fn length(&self) -> i64 {
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
    /// assert_eq!(enclosure_type.to_owned(), enclosure.mime_type())
    /// ```
    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }
}
