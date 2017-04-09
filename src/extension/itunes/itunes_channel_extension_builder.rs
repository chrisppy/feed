// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for itunes category by using the methods under
//! `ITunesChannelExtensionBuilder`.


use extension::itunes::ITunesChannelExtensionBuilder;
use rss::extension::itunes::{ITunesCategory, ITunesChannelExtension, ITunesOwner};


impl ITunesChannelExtensionBuilder
{
    /// Construct a new `ITunesChannelExtension` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new();
    /// ```
    pub fn new() -> ITunesChannelExtensionBuilder
    {
        ITunesChannelExtensionBuilder::default()
    }


    /// Set the optional author that exists under `ITunesChannelExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.author(Some("author".to_owned()));
    /// ```
    pub fn author(&mut self, author: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional block that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.block(Some("block".to_owned()));
    /// ```
    pub fn block(&mut self, block: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.block = block;
        self
    }


    /// Set the categories that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesChannelExtensionBuilder};
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = vec![category];
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.categories(categories);
    /// ```
    pub fn categories(&mut self, categories: Vec<ITunesCategory>) -> &mut ITunesChannelExtensionBuilder
    {
        self.categories = categories;
        self
    }


    /// Set the optional image that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.image(Some("image".to_owned()));
    /// ```
    pub fn image(&mut self, image: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.image = image;
        self
    }


    /// Set the optional explicit that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.explicit(Some("explicit".to_owned()));
    /// ```
    pub fn explicit(&mut self, explicit: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    /// Set the optional complete that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.complete(Some("complete".to_owned()));
    /// ```
    pub fn complete(&mut self, complete: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.complete = complete;
        self
    }


    /// Set the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.new_feed_url(Some("new_feed_url".to_owned()));
    /// ```
    pub fn new_feed_url(&mut self, new_feed_url: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.new_feed_url = new_feed_url;
        self
    }


    /// Set the optional owner that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.owner(Some(owner));
    /// ```
    pub fn owner(&mut self, owner: Option<ITunesOwner>) -> &mut ITunesChannelExtensionBuilder
    {
        self.owner = owner;
        self
    }


    /// Set the optional subtitle that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.subtitle(Some("subtitle".to_owned()));
    /// ```
    pub fn subtitle(&mut self, subtitle: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    /// Set the optional summary that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.summary(Some("summary".to_owned()));
    /// ```
    pub fn summary(&mut self, summary: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.summary = summary;
        self
    }


    /// Set the optional keywords that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.keywords(Some("keywords".to_owned()));
    /// ```
    pub fn keywords(&mut self, keywords: Option<String>) -> &mut ITunesChannelExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesChannelExtension` from the
    /// `ITunesChannelExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesChannelExtensionBuilder, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = vec![category];
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .new_feed_url(Some("new_feed_url".to_owned()))
    ///     .complete(Some("complete".to_owned()))
    ///     .owner(Some(owner))
    ///     .categories(categories)
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<ITunesChannelExtension, String>
    {
        Ok(ITunesChannelExtension {
               author: self.author.clone(),
               block: self.block.clone(),
               categories: self.categories.clone(),
               image: self.image.clone(),
               explicit: self.explicit.clone(),
               complete: self.complete.clone(),
               new_feed_url: self.new_feed_url.clone(),
               owner: self.owner.clone(),
               subtitle: self.subtitle.clone(),
               summary: self.summary.clone(),
               keywords: self.keywords.clone(),
           })
    }
}
