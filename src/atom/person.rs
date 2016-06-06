// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under person can be retrieved by using the methods under `Person`.

use atom::Person;
use url::Url;

impl Person {
    /// Get the name that exists under `Person`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::PersonBuilder;
    ///
    /// let name = "Chris Krycho";
    /// let person = PersonBuilder::new()
    ///     .name(name)
    ///     .finalize();
    /// assert_eq!(name.to_owned(), person.name());
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }


    /// Get the uri that exists under `Person`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::PersonBuilder;
    ///
    /// fn main() {
    ///     let uri = "http://www.chriskrycho.com/".to_owned();
    ///     let person = PersonBuilder::new()
    ///         .uri(Some(uri.clone()))
    ///         .finalize();
    ///     assert_eq!(uri.clone(), person.uri().unwrap().into_string());
    /// }
    /// ```
    pub fn uri(&self) -> Option<Url> {
        self.uri.clone()
    }


    /// Get the email that exists under `Person`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::PersonBuilder;
    ///
    /// let email = "chris@chriskrycho.com";
    /// let person = PersonBuilder::new()
    ///     .email(Some(email.to_owned()))
    ///     .finalize();
    /// assert_eq!(Some(email.to_owned()), person.email());
    /// ```
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
}
