// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for person by using the methods under `PersonBuilder`.

use atom::{Person, PersonBuilder};
use errors;
use url::Url;

impl PersonBuilder {
    /// Construct a new `PersonBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::PersonBuilder;
    ///
    /// let person_builder = PersonBuilder::new();
    /// ```
    pub fn new() -> PersonBuilder {
        PersonBuilder::default()
    }


    /// Set the name that exists under `Person`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::PersonBuilder;
    ///
    /// let mut person_builder = PersonBuilder::new();
    /// person_builder.name("Chris Krycho");
    /// ```
    pub fn name(&mut self, name: &str) -> &mut PersonBuilder {
        self.name = name.to_owned();
        self
    }


    /// Set the uri that exists under `Person`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    /// extern crate url;
    ///
    /// use feed::atom::PersonBuilder;
    /// use url::Url;
    ///
    /// fn main() {
    ///     let mut person_builder = PersonBuilder::new();
    ///     person_builder.uri(Some("http://www.chriskrycho.com".to_owned()));
    /// }
    /// ```
    pub fn uri(&mut self, uri: Option<String>) -> &mut PersonBuilder {
        self.uri = uri;
        self
    }


    /// Set the email that exists under `Person`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::PersonBuilder;
    ///
    /// let mut person_builder = PersonBuilder::new();
    /// person_builder.email(Some("chris@chriskrycho.com".to_owned()));
    /// ```
    pub fn email(&mut self, email: Option<String>) -> &mut PersonBuilder {
        self.email = email;
        self
    }


    /// Construct the `Person` from the `PersonBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::PersonBuilder;
    ///
    /// fn main() {
    ///     let person = PersonBuilder::new()
    ///         .name("Chris Krycho")
    ///         .uri(Some("http://www.chriskrycho.com".to_owned()))
    ///         .email(Some("chris@chriskrycho.com".to_owned()))
    ///         .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> Person {
        if self.name.is_empty() {
            panic!(errors::empty_string_error("Atom Person name"));
        }

        let mut uri_option: Option<Url> = None;
        if self.uri.is_some() {
            let uri = self.uri.clone().unwrap();
            let url = Url::parse(uri.as_str())
                .expect(errors::url_parse_error(uri.as_str()).as_str());
            uri_option = Some(url);
        }
        Person {
            name: self.name.clone(),
            uri: uri_option,
            email: self.email.clone(),
        }
    }
}
