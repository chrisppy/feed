// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for enclosure by using the methods under `EnclosureBuilder`.


use errors;
use channels::{Enclosure, EnclosureBuilder};


impl EnclosureBuilder {
    /// Construct a new `EnclosureBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::EnclosureBuilder;
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
    /// use feed::channels::EnclosureBuilder;
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
    /// use feed::channels::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.length(70772893);
    /// ```
    pub fn length(&mut self, length: i64) -> &mut EnclosureBuilder {
        if length < 0 {
            panic!(errors::negative_error("enclosure length", length));
        }
        self.length = length;
        self
    }


    /// Set the enclosure_type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::EnclosureBuilder;
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
    /// use feed::channels::EnclosureBuilder;
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
