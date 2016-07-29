

// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under generator can be retrieved by using the methods under `Generator`.

use atom::Generator;
use url::Url;

impl Generator {
    /// Get the generator that exists under `Generator`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let generator_str = "Example Toolkit";
    ///     let generator = GeneratorBuilder::new()
    ///         .generator(generator_str)
    ///         .finalize();
    ///     assert_eq!(generator_str.to_owned(), generator.generator());
    /// }
    /// ```
    pub fn generator(&self) -> String {
        self.generator.clone()
    }


    /// Get the uri that exists under `Generator`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let uri = "http://www.example.com/myblog.php".to_owned();
    ///     let generator = GeneratorBuilder::new()
    ///         .generator("Example Toolkit")
    ///         .uri(Some(uri.clone()))
    ///         .finalize();
    ///     assert_eq!(uri.clone(), generator.uri().unwrap().into_string());
    /// }
    /// ```
    pub fn uri(&self) -> Option<Url> {
        self.uri.clone()
    }


    /// Get the version that exists under `Generator`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::GeneratorBuilder;
    ///
    /// fn main() {
    ///     let version = "General";
    ///     let generator = GeneratorBuilder::new()
    ///         .generator("Example Toolkit")
    ///         .version(Some(version.to_owned()))
    ///         .finalize();
    ///     assert_eq!(Some(version.to_owned()), generator.version());
    /// }
    /// ```
    pub fn version(&self) -> Option<String> {
        self.version.clone()
    }
}
