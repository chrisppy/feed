// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for item by using the methods under `ItemBuilder`.


use ItemBuilder;
use rss::{Category, Enclosure, Guid, Item, Source};
use rss::extension::itunes::ITunesItemExtension;
use utils::string_utils;


impl ItemBuilder
{
    /// Construct a new `ItemBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new();
    /// ```
    pub fn new() -> ItemBuilder
    {
        ItemBuilder::default()
    }


    /// Set the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.title(Some("Making Music with Linux | LAS
    /// 408".to_owned()));
    /// ```
    pub fn title(&mut self, title: Option<String>) -> &mut ItemBuilder
    {
        self.title = title;
        self
    }


    /// Set the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.link(Some("http://www.jupiterbroadcasting.com".
    /// to_owned()));
    /// ```
    pub fn link(&mut self, link: Option<String>) -> &mut ItemBuilder
    {
        self.link = link;
        self
    }


    /// Set the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.description(Some("This is a test description".to_owned()));
    /// ```
    pub fn description(&mut self, description: Option<String>) -> &mut ItemBuilder
    {
        self.description = description;
        self
    }


    /// Set the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.author(Some("Chris Fisher".to_owned()));
    /// ```
    pub fn author(&mut self, author: Option<String>) -> &mut ItemBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{CategoryBuilder, ItemBuilder};
    ///
    /// let category = CategoryBuilder::new()
    ///     .finalize()
    ///     .unwrap();;
    /// let categories = vec![category];
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.categories(categories);
    /// ```
    pub fn categories(&mut self, categories: Vec<Category>) -> &mut ItemBuilder
    {
        self.categories = categories;
        self
    }


    /// Set the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.comments(Some("Test Comment".to_owned()));
    /// ```
    pub fn comments(&mut self, comments: Option<String>) -> &mut ItemBuilder
    {
        self.comments = comments;
        self
    }


    /// Set the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.enclosure(Some(enclosure));
    /// ```
    pub fn enclosure(&mut self, enclosure: Option<Enclosure>) -> &mut ItemBuilder
    {
        self.enclosure = enclosure;
        self
    }


    /// Set the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::new()
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.guid(Some(guid));
    /// ```
    pub fn guid(&mut self, guid: Option<Guid>) -> &mut ItemBuilder
    {
        self.guid = guid;
        self
    }


    /// Set the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.pub_date(Some("Sun, 13 Mar 2016
    /// 20:02:02-0700".to_owned()));
    /// ```
    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ItemBuilder
    {
        self.pub_date = pub_date;
        self
    }


    /// Set the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{ItemBuilder, SourceBuilder};
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source = SourceBuilder::new()
    ///     .url(url)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.source(Some(source));
    /// ```
    pub fn source(&mut self, source: Option<Source>) -> &mut ItemBuilder
    {
        self.source = source;
        self
    }


    /// TODO
    pub fn itunes_ext(&mut self, itunes_ext: Option<ITunesItemExtension>) -> &mut ItemBuilder
    {
        self.itunes_ext = itunes_ext;
        self
    }


    /// Validate the contents of `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some("http://www.jupiterbroadcasting.com".to_owned()))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .comments(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .validate().unwrap()
    ///     .finalize().unwrap();
    /// ```
    pub fn validate(&mut self) -> Result<&mut ItemBuilder, String>
    {
        if self.title.is_none() && self.description.is_none()
        {
            return Err("Either Title or Description must have a value.".to_owned());
        }

        let link = self.link.clone();
        if link.is_some()
        {
            string_utils::str_to_url(link.unwrap().as_str())?;
        }

        let comments = self.comments.clone();
        if comments.is_some()
        {
            string_utils::str_to_url(comments.unwrap().as_str())?;
        }

        string_utils::option_string_to_option_date(self.pub_date.clone())?;

        Ok(self)
    }


    /// Construct the `Item` from the `ItemBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some("http://www.jupiterbroadcasting.com".to_owned()))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .comments(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<Item, String>
    {
        Ok(Item {
               title: self.title.clone(),
               link: self.link.clone(),
               description: self.description.clone(),
               author: self.author.clone(),
               categories: self.categories.clone(),
               comments: self.comments.clone(),
               enclosure: self.enclosure.clone(),
               guid: self.guid.clone(),
               pub_date: self.pub_date.clone(),
               source: self.source.clone(),
               itunes_ext: self.itunes_ext.clone(),
               ..Default::default()
           })
    }
}
