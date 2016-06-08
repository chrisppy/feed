// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for feed by using the methods under `AtomFeedBuilder`.

use atom::{AtomFeed, AtomFeedBuilder, AtomCategory, Entry, Generator, Link, Person, Text};
use chrono::*;
use errors;
use url::Url;

impl AtomFeedBuilder {
    /// Construct a new `AtomFeedBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// let feed_builder = AtomFeedBuilder::new();
    /// ```
    pub fn new() -> AtomFeedBuilder {
        AtomFeedBuilder::default()
    }


    /// Set the id that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// fn main() {
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.id("http://newrustacean.com/show_notes/e014/");
    /// }
    /// ```
    pub fn id(&mut self, id: &str) -> &mut AtomFeedBuilder {
        self.id = id.to_owned();
        self
    }


    /// Set the title that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// fn main() {
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.title("e014: Stringing things along");
    /// }
    /// ```
    pub fn title(&mut self, title: Text) -> &mut AtomFeedBuilder {
        self.title = title;
        self
    }


    /// Set the updated that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// fn main() {
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.updated("2014-11-28T12:00:09+00:00");
    /// }
    /// ```
    pub fn updated(&mut self, updated: &str) -> &mut AtomFeedBuilder {
        self.updated = updated.to_owned();
        self
    }


    /// Set the authors that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, PersonBuilder};
    ///
    /// fn main() {
    ///     let author = PersonBuilder::new().name("Chris Krycho").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.authors(Some(vec![author]));
    /// }
    /// ```
    pub fn authors(&mut self, authors: Option<Vec<Person>>) -> &mut AtomFeedBuilder {
        self.authors = authors;
        self
    }


    /// Set the links that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, LinkBuilder};
    ///
    /// fn main() {
    ///     let link = LinkBuilder::new().href("http://www.example.com").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.links(Some(vec![link]));
    /// }
    /// ```
    pub fn links(&mut self, links: Option<Vec<Link>>) -> &mut AtomFeedBuilder {
        self.links = links;
        self
    }


    /// Set the categories that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, AtomCategoryBuilder};
    ///
    /// fn main() {
    ///     let category = AtomCategoryBuilder::new().term("Tech").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.categories(Some(vec![category]));
    /// }
    /// ```
    pub fn categories(&mut self, categories: Option<Vec<AtomCategory>>) -> &mut AtomFeedBuilder {
        self.categories = categories;
        self
    }


    /// Set the contributors that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, PersonBuilder};
    ///
    /// fn main() {
    ///     let contributor = PersonBuilder::new().name("Chris Krycho").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.contributors(Some(vec![contributor]));
    /// }
    /// ```
    pub fn contributors(&mut self, contributors: Option<Vec<Person>>) -> &mut AtomFeedBuilder {
        self.contributors = contributors;
        self
    }


    /// Set the generator that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, GeneratorBuilder};
    ///
    /// fn main() {
    ///     let generator = GeneratorBuilder::new().generator("Example Toolkit").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.generator(Some(generator));
    /// }
    /// ```
    pub fn generator(&mut self, generator: Option<Generator>) -> &mut AtomFeedBuilder {
        self.generator = generator;
        self
    }


    /// Set the icon that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// fn main() {
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.icon(Some("http://example.com/image.png".to_owned()));
    /// }
    /// ```
    pub fn icon(&mut self, icon: Option<String>) -> &mut AtomFeedBuilder {
        self.icon = icon;
        self
    }


    /// Set the logo that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// fn main() {
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.logo(Some("http://example.com/image.png".to_owned()));
    /// }
    /// ```
    pub fn logo(&mut self, logo: Option<String>) -> &mut AtomFeedBuilder {
        self.logo = logo;
        self
    }


    /// Set the rights that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let rights = TextBuilder::new().text("© 2005 John Doe").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.rights(Some(rights));
    /// }
    /// ```
    pub fn rights(&mut self, rights: Option<Text>) -> &mut AtomFeedBuilder {
        self.rights = rights;
        self
    }


    /// Set the subtitle that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, TextBuilder};
    ///
    /// fn main() {
    ///     let subtitle = TextBuilder::new().text("Example").finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.subtitle(Some(subtitle));
    /// }
    /// ```
    pub fn subtitle(&mut self, subtitle: Option<Text>) -> &mut AtomFeedBuilder {
        self.subtitle = subtitle;
        self
    }


    /// Set the entries that exists under `AtomFeed`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::{AtomFeedBuilder, EntryBuilder};
    ///
    /// fn main() {
    ///     let entry = EntryBuilder::new()
    ///         .id("http://newrustacean.com/show_notes/e014/")
    ///         .title("e014: Stringing things along")
    ///         .updated("2014-11-28T12:00:09+00:00")
    ///         .finalize();
    ///     let mut feed_builder = AtomFeedBuilder::new();
    ///     feed_builder.entries(Some(vec![entry]));
    /// }
    /// ```
    pub fn entries(&mut self, entries: Option<Vec<Entry>>) -> &mut AtomFeedBuilder {
        self.entries = entries;
        self
    }


    /// Construct the `AtomFeed` from the `AtomFeedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    ///
    /// use feed::atom::AtomFeedBuilder;
    ///
    /// fn main() {
    ///     let feed = AtomFeedBuilder::new()
    ///     .id("http://newrustacean.com/show_notes/e014/")
    ///     .title("e014: Stringing things along")
    ///     .updated("2014-11-28T12:00:09+00:00")
    ///     .finalize();
    /// }
    /// ```
    pub fn finalize(&self) -> AtomFeed {
        let mut icon_option: Option<Url> = None;
        if self.icon.is_some() {
            let url = Url::parse(self.icon.clone().unwrap().as_str())
                .expect(errors::url_parse_error(self.icon.clone().unwrap().as_str()).as_str());
            icon_option = Some(url);
        }

        let mut logo_option: Option<Url> = None;
        if self.logo.is_some() {
            let url = Url::parse(self.logo.clone().unwrap().as_str())
                .expect(errors::url_parse_error(self.logo.clone().unwrap().as_str()).as_str());
            logo_option = Some(url);
        }

        AtomFeed {
            id: Url::parse(self.id.clone().as_str())
                .expect(errors::url_parse_error(self.id.clone().as_str()).as_str()),
            title: self.title.clone(),
            updated: DateTime::parse_from_rfc3339(self.updated.clone().as_str())
                .expect(errors::date_parse_error(self.updated.clone().as_str()).as_str()),
            authors: self.authors.clone(),
            links: self.links.clone(),
            categories: self.categories.clone(),
            contributors: self.contributors.clone(),
            generator: self.generator.clone(),
            icon: icon_option,
            logo: logo_option,
            rights: self.rights.clone(),
            subtitle: self.subtitle.clone(),
            entries: self.entries.clone(),
        }
    }
}
