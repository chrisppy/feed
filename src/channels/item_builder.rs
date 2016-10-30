// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for item by using the methods under `ItemBuilder`.


use channels::{Category, Enclosure, Guid, Item, ItemBuilder, Source};
use utils::string_utils;
use errors;


impl ItemBuilder {
    /// Construct a new `ItemBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new();
    /// ```
    pub fn new() -> ItemBuilder {
        ItemBuilder::default()
    }


    /// Set the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.title(Some("Making Music with Linux | LAS 408".to_owned()));
    /// ```
    pub fn title(&mut self, title: Option<String>) -> &mut ItemBuilder {
        self.title = title;
        self
    }


    /// Set the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.link(Some("http://www.jupiterbroadcasting.com".to_owned()));
    /// ```
    pub fn link(&mut self, link: Option<String>) -> &mut ItemBuilder {
        self.link = link;
        self
    }


    /// Set the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.description(Some("This is a test description".to_owned()));
    /// ```
    pub fn description(&mut self, description: Option<String>) -> &mut ItemBuilder {
        self.description = description;
        self
    }


    /// Set the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.author(Some("Chris Fisher".to_owned()));
    /// ```
    pub fn author(&mut self, author: Option<String>) -> &mut ItemBuilder {
        self.author = author;
        self
    }


    /// Set the optional categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{CategoryBuilder, ItemBuilder};
    ///
    /// let category = CategoryBuilder::new().finalize();
    /// let categories = vec![category];
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.categories(Some(categories));
    /// ```
    pub fn categories(&mut self, categories: Option<Vec<Category>>) -> &mut ItemBuilder {
        self.categories = categories;
        self
    }


    /// Set the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.comments(Some("Test Comment".to_owned()));
    /// ```
    pub fn comments(&mut self, comments: Option<String>) -> &mut ItemBuilder {
        self.comments = comments;
        self
    }


    /// Set the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .finalize();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.enclosure(Some(enclosure));
    /// ```
    pub fn enclosure(&mut self, enclosure: Option<Enclosure>) -> &mut ItemBuilder {
        self.enclosure = enclosure;
        self
    }


    /// Set the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::new().finalize();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.guid(Some(guid));
    /// ```
    pub fn guid(&mut self, guid: Option<Guid>) -> &mut ItemBuilder {
        self.guid = guid;
        self
    }


    /// Set the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.pub_date(Some("Sun, 13 Mar 2016 20:02:02 -0700".to_owned()));
    /// ```
    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ItemBuilder {
        self.pub_date = string_utils::option_string_to_option_date(pub_date);
        self
    }


    /// Set the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ItemBuilder, SourceBuilder};
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source = SourceBuilder::new()
    ///     .url(url)
    ///     .finalize();
    ///
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.source(Some(source));
    /// ```
    pub fn source(&mut self, source: Option<Source>) -> &mut ItemBuilder {
        self.source = source;
        self
    }


    /// Construct the `Item` from the `ItemBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///         .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///         .link(Some("http://www.jupiterbroadcasting.com".to_owned()))
    ///         .description(None)
    ///         .author(None)
    ///         .categories(None)
    ///         .comments(None)
    ///         .enclosure(None)
    ///         .guid(None)
    ///         .pub_date(None)
    ///         .source(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Item {
        if self.title.is_none() && self.description.is_none() {
            panic!(errors::item_required_field_error());
        }

        let link_opt = self.link.clone();
        let link = if link_opt.is_none() {
            None
        } else {
            let link_string = link_opt.clone().unwrap();
            let url = string_utils::str_to_url(link_string.as_str(), "Item Link");
            Some(url)
        };

        let comments_opt = self.comments.clone();
        let comments = if comments_opt.is_none() {
            None
        } else {
            let comments_string = comments_opt.clone().unwrap();
            let url = string_utils::str_to_url(comments_string.as_str(), "Item Comments");
            Some(url)
        };

        Item {
            title: self.title.clone(),
            link: link,
            description: self.description.clone(),
            author: self.author.clone(),
            categories: self.categories.clone(),
            comments: comments,
            enclosure: self.enclosure.clone(),
            guid: self.guid.clone(),
            pub_date: self.pub_date,
            source: self.source.clone(),
        }
    }
}
