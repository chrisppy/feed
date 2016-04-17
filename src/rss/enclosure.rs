// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under enclosure can be retrieved by using the methods under `Enclosure`
//! and the fields can be set for enclosure by using the methods under `EnclosureBuilder`.

use rss::{Enclosure, EnclosureBuilder};

impl Enclosure {
    /// Get the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .finalize();
    /// assert_eq!(url.to_owned(), enclosure.url())
    /// ```
    pub fn url(&self) -> String {
        self.url.clone()
    }


    /// Get the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let length: i64 = 70772893;
    /// let enclosure = EnclosureBuilder::new()
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
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let enclosure_type = "audio/ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///     .enclosure_type(enclosure_type)
    ///     .finalize();
    /// assert_eq!(enclosure_type.to_owned(), enclosure.enclosure_type())
    /// ```
    pub fn enclosure_type(&self) -> String {
        self.enclosure_type.clone()
    }
}


impl EnclosureBuilder {
    /// Construct a new `EnclosureBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let enclosure_builder = EnclosureBuilder::new();
    /// ```
    pub fn new() -> EnclosureBuilder {
        EnclosureBuilder::default()
    }


    /// Set the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/".to_owned()
    /// + "redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.url(url.as_ref());
    /// ```
    pub fn url(&mut self, url: &str) -> &mut EnclosureBuilder {
        self.url = url.to_owned();
        self
    }


    /// Set the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.length(70772893);
    /// ```
    pub fn length(&mut self, length: i64) -> &mut EnclosureBuilder {
        self.length = length;
        self
    }


    /// Set the enclosure_type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.enclosure_type("audio/ogg");
    /// ```
    pub fn enclosure_type(&mut self, enclosure_type: &str) -> &mut EnclosureBuilder {
        self.enclosure_type = enclosure_type.to_owned();
        self
    }


    /// Construct the `Enclosure` from the `EnclosureBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///         .url(url.as_ref())
    ///         .length(70772893)
    ///         .enclosure_type("audio/ogg")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Enclosure {
        Enclosure {
            url: self.url.clone(),
            length: self.length,
            enclosure_type: self.enclosure_type.clone(),
        }
    }
}
