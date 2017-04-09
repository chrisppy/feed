// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under itunes category can be retrieved by using the methods under
//! `ITunesChannelExtension`.


use extension::itunes::ITunesChannelExtensionGetters;
use rss::extension::itunes::{ITunesCategory, ITunesChannelExtension, ITunesOwner};


impl ITunesChannelExtensionGetters for ITunesChannelExtension
{
    /// Get the optional author that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let author = "author".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some(author.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = channel.author();
    /// assert!(author_opt.is_some());
    ///
    /// assert_eq!(author.clone(), author_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = channel.author();
    /// assert!(author_opt.is_none());
    /// ```
    fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    /// Get the optional block that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let block = "block".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .block(Some(block.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = channel.block();
    /// assert!(block_opt.is_some());
    ///
    /// assert_eq!(block.clone(), block_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .block(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = channel.block();
    /// assert!(block_opt.is_none());
    /// ```
    fn block(&self) -> Option<String>
    {
        self.block.clone()
    }


    /// Get the categories that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters, ITunesCategoryBuilder};
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
    /// let categories_vec = vec![category];
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .categories(categories_vec)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = channel.categories();
    /// assert!(!categories.is_empty());
    /// ```
    fn categories(&self) -> Vec<ITunesCategory>
    {
        self.categories.clone()
    }


    /// Get the optional image that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let image = "image".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .image(Some(image.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = channel.image();
    /// assert!(image_opt.is_some());
    ///
    /// assert_eq!(image.clone(), image_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .image(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = channel.image();
    /// assert!(image_opt.is_none());
    /// ```
    fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    /// Get the optional explicit that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let explicit = "explicit".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .explicit(Some(explicit.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = channel.explicit();
    /// assert!(explicit_opt.is_some());
    ///
    /// assert_eq!(explicit.clone(), explicit_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .explicit(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = channel.explicit();
    /// assert!(explicit_opt.is_none());
    /// ```
    fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    /// Get the optional complete that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let complete = "complete".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .complete(Some(complete.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let complete_opt = channel.complete();
    /// assert!(complete_opt.is_some());
    ///
    /// assert_eq!(complete.clone(), complete_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .complete(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let complete_opt = channel.complete();
    /// assert!(complete_opt.is_none());
    /// ```
    fn complete(&self) -> Option<String>
    {
        self.complete.clone()
    }


    /// Get the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let new_feed_url = "new_feed_url".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url(Some(new_feed_url.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let new_feed_url_opt = channel.new_feed_url();
    /// assert!(new_feed_url_opt.is_some());
    ///
    /// assert_eq!(new_feed_url.clone(), new_feed_url_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let new_feed_url_opt = channel.new_feed_url();
    /// assert!(new_feed_url_opt.is_none());
    /// ```
    fn new_feed_url(&self) -> Option<String>
    {
        self.new_feed_url.clone()
    }


    /// Get the optional owner that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .owner(Some(owner))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let owner_opt = channel.owner();
    /// assert!(owner_opt.is_some());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .owner(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let owner_opt = channel.owner();
    /// assert!(owner_opt.is_none());
    /// ```
    fn owner(&self) -> Option<ITunesOwner>
    {
        self.owner.clone()
    }


    /// Get the optional subtitle that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let subtitle = "subtitle".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .subtitle(Some(subtitle.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = channel.subtitle();
    /// assert!(subtitle_opt.is_some());
    ///
    /// assert_eq!(subtitle.clone(), subtitle_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .subtitle(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = channel.subtitle();
    /// assert!(subtitle_opt.is_none());
    /// ```
    fn subtitle(&self) -> Option<String>
    {
        self.subtitle.clone()
    }


    /// Get the optional summary that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let summary = "summary".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .summary(Some(summary.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = channel.summary();
    /// assert!(summary_opt.is_some());
    ///
    /// assert_eq!(summary.clone(), summary_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .summary(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = channel.summary();
    /// assert!(summary_opt.is_none());
    /// ```
    fn summary(&self) -> Option<String>
    {
        self.summary.clone()
    }


    /// Get the optional keywords that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let keywords = "keywords".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .keywords(Some(keywords.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = channel.keywords();
    /// assert!(keywords_opt.is_some());
    ///
    /// assert_eq!(keywords.clone(), keywords_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtensionGetters};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .keywords(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = channel.keywords();
    /// assert!(keywords_opt.is_none());
    /// ```
    fn keywords(&self) -> Option<String>
    {
        self.keywords.clone()
    }
}
