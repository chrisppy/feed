// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for itunes category by using the methods under
//! `ITunesItemExtensionBuilder`.


use extension::itunes::ITunesItemExtensionBuilder;
use rss::extension::itunes::ITunesItemExtension;


impl ITunesItemExtensionBuilder
{
    /// Construct a new `ITunesItemExtensionBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item_builder = ITunesItemExtensionBuilder::new();
    /// ```
    pub fn new() -> ITunesItemExtensionBuilder
    {
        ITunesItemExtensionBuilder::default()
    }


    /// Set the optional author that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.author(Some("author".to_owned()));
    /// ```
    pub fn author(&mut self, author: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional block that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.block(Some("block".to_owned()));
    /// ```
    pub fn block(&mut self, block: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.block = block;
        self
    }


    /// Set the optional image that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.image(Some("image".to_owned()));
    /// ```
    pub fn image(&mut self, image: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.image = image;
        self
    }


    /// Set the optional duration that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.duration(Some("duration".to_owned()));
    /// ```
    pub fn duration(&mut self, duration: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.duration = duration;
        self
    }


    /// Set the optional explicit that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.explicit(Some("explicit".to_owned()));
    /// ```
    pub fn explicit(&mut self, explicit: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    /// Set the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.closed_captioned(Some("closed_captioned".to_owned()));
    /// ```
    pub fn closed_captioned(&mut self, closed_captioned: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.closed_captioned = closed_captioned;
        self
    }


    /// Set the optional order that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.order(Some("order".to_owned()));
    /// ```
    pub fn order(&mut self, order: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.order = order;
        self
    }


    /// Set the optional subtitle that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.subtitle(Some("subtitle".to_owned()));
    /// ```
    pub fn subtitle(&mut self, subtitle: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    /// Set the optional summary that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.summary(Some("summary".to_owned()));
    /// ```
    pub fn summary(&mut self, summary: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.summary = summary;
        self
    }


    /// Set the optional keywords that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.keywords(Some("keywords".to_owned()));
    /// ```
    pub fn keywords(&mut self, keywords: Option<String>) -> &mut ITunesItemExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesItemExtension` from the
    /// `ITunesItemExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .duration(Some("duration".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .closed_captioned(Some("closed_captioned".to_owned()))
    ///     .order(Some("order".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<ITunesItemExtension, String>
    {
        Ok(ITunesItemExtension {
               author: self.author.clone(),
               block: self.block.clone(),
               image: self.image.clone(),
               duration: self.duration.clone(),
               explicit: self.explicit.clone(),
               closed_captioned: self.closed_captioned.clone(),
               order: self.order.clone(),
               subtitle: self.subtitle.clone(),
               summary: self.summary.clone(),
               keywords: self.keywords.clone(),
           })
    }
}
