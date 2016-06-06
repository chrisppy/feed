
// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for generator by using the methods under `GeneratorBuilder`.

use atom::{Generator, GeneratorBuilder};
use errors;
use url::Url;

impl GeneratorBuilder {
    /// Construct a new `GeneratorBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::GeneratorBuilder;
    ///
    /// let generator_builder = GeneratorBuilder::new();
    /// ```
    pub fn new() -> GeneratorBuilder {
        GeneratorBuilder::default()
    }


    /// Set the generator that exists under `Generator`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let mut generator_builder = GeneratorBuilder::new();
    ///     generator_builder.generator("Example Toolkit");
    /// }
    /// ```
    pub fn generator(&mut self, generator: &str) -> &mut GeneratorBuilder {
        self.generator = generator.to_owned();
        self
    }


    /// Set the uri that exists under `Generator`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let mut generator_builder = GeneratorBuilder::new();
    ///     generator_builder.uri(Some("http://www.example.com/myblog.php".to_owned()));
    /// }
    /// ```
    pub fn uri(&mut self, uri: Option<String>) -> &mut GeneratorBuilder {
        self.uri = uri;
        self
    }


    /// Set the version that exists under `Generator`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let version = "1.0";
    ///     let mut generator_builder = GeneratorBuilder::new();
    ///     generator_builder.version(Some(version.to_owned()));
    /// }
    /// ```
    pub fn version(&mut self, version: Option<String>) -> &mut GeneratorBuilder {
        self.version = version;
        self
    }


    /// Construct the `Generator` from the `GeneratorBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let generator = GeneratorBuilder::new()
    ///         .generator("Example Toolkit")
    ///         .uri(Some("http://www.example.com/myblog.php".to_owned()))
    ///         .version(Some("1.0".to_owned()))
    ///         .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> Generator {
        let mut uri_option: Option<Url> = None;
        if self.uri.is_some() {
            let uri = self.uri.clone().unwrap();
            let url = Url::parse(uri.as_str())
                          .expect(errors::url_parse_error(uri.as_str()).as_str());
            uri_option = Some(url);
        }
        Generator {
            generator: self.generator.clone(),
            uri: uri_option,
            version: self.version.clone(),
        }
    }
}
