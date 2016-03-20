// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

/// This `Enclosure` struct contains all the items that exist for the enclosure field under 'Item'.
#[derive(Clone)]
pub struct Enclosure {
    url:            String,
    length:         i64,
    enclosure_type: String,
}


impl Enclosure {
    /// Get the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url)
    ///     .finalize();
    /// assert_eq!(url.to_string(), enclosure.url())
    /// ```
    pub fn url(&self) -> String {
        self.url.clone()
    }


    /// Get the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let length: i64 = 70772893;
    /// let enclosure = EnclosureBuilder::new()
    ///     .length(length)
    ///     .finalize();
    /// assert_eq!(length, enclosure.length())
    /// ```
    pub fn length(&self) -> i64 {
        self.length.clone()
    }


    /// Get the enclosure type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let enclosure_type = "audio/ogg";
    /// let enclosure = EnclosureBuilder::new()
    ///     .enclosure_type(enclosure_type)
    ///     .finalize();
    /// assert_eq!(enclosure_type.to_string(), enclosure.enclosure_type())
    /// ```
    pub fn enclosure_type(&self) -> String {
        self.enclosure_type.clone()
    }
}


/// This `EnclosureBuilder` struct creates the `Enclosure`.
pub struct EnclosureBuilder {
    url:            String,
    length:         i64,
    enclosure_type: String,
}


impl EnclosureBuilder {
    /// Construct a new `EnclosureBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let enclosure_builder = EnclosureBuilder::new();
    /// ```
    pub fn new() -> EnclosureBuilder {
        EnclosureBuilder {
            url:            String::new(),
            length:         0,
            enclosure_type: String::new(),
        }
    }


    /// Set the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.url("http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg");
    /// ```
    pub fn url(&mut self, url: &str) -> &mut EnclosureBuilder {
        self.url = url.to_string();
        self
    }


    /// Set the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
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
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.enclosure_type("audio/ogg");
    /// ```
    pub fn enclosure_type(&mut self, enclosure_type: &str) -> &mut EnclosureBuilder {
        self.enclosure_type = enclosure_type.to_string();
        self
    }


    /// Construct the `Enclosure` from the `EnclosureBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///         .url("http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg")
    ///         .length(70772893)
    ///         .enclosure_type("audio/ogg")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Enclosure {
        Enclosure {
            url:            self.url.clone(),
            length:         self.length.clone(),
            enclosure_type: self.enclosure_type.clone(),
        }
    }
}
