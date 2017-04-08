// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under itunes category can be retrieved by using the methods under
//! `ITunesItemExtension`.


use extension::itunes::ITunesItemExtensionGetters;
use rss::extension::itunes::ITunesItemExtension;


impl ITunesItemExtensionGetters for ITunesItemExtension
{
    /// Get the optional author that exists under `ITunesItemExtension`.
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let author = "author".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(Some(author.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = item.author();
    /// assert!(author_opt.is_some());
    ///
    /// assert_eq!(author.clone(), author_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = item.author();
    /// assert!(author_opt.is_none());
    /// ```
    fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    /// Get the optional block that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let block = "block".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .block(Some(block.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = item.block();
    /// assert!(block_opt.is_some());
    ///
    /// assert_eq!(block.clone(), block_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .block(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = item.block();
    /// assert!(block_opt.is_none());
    /// ```
    fn block(&self) -> Option<String>
    {
        self.block.clone()
    }


    /// Get the optional image that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let image = "image".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .image(Some(image.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = item.image();
    /// assert!(image_opt.is_some());
    ///
    /// assert_eq!(image.clone(), image_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .image(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = item.image();
    /// assert!(image_opt.is_none());
    /// ```
    fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    /// Get the optional duration that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let duration = "duration".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .duration(Some(duration.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let duration_opt = item.duration();
    /// assert!(duration_opt.is_some());
    ///
    /// assert_eq!(duration.clone(), duration_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .duration(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let duration_opt = item.duration();
    /// assert!(duration_opt.is_none());
    /// ```
    fn duration(&self) -> Option<String>
    {
        self.duration.clone()
    }


    /// Get the optional explicit that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let explicit = "explicit".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .explicit(Some(explicit.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = item.explicit();
    /// assert!(explicit_opt.is_some());
    ///
    /// assert_eq!(explicit.clone(), explicit_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .explicit(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = item.explicit();
    /// assert!(explicit_opt.is_none());
    /// ```
    fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    /// Get the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let closed_captioned = "closed_captioned".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(Some(closed_captioned.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let closed_captioned_opt = item.closed_captioned();
    /// assert!(closed_captioned_opt.is_some());
    ///
    /// assert_eq!(closed_captioned.clone(), closed_captioned_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let closed_captioned_opt = item.closed_captioned();
    /// assert!(closed_captioned_opt.is_none());
    /// ```
    fn closed_captioned(&self) -> Option<String>
    {
        self.closed_captioned.clone()
    }


    /// Get the optional order that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let order = "order".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .order(Some(order.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let order_opt = item.order();
    /// assert!(order_opt.is_some());
    ///
    /// assert_eq!(order.clone(), order_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .order(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let order_opt = item.order();
    /// assert!(order_opt.is_none());
    /// ```
    fn order(&self) -> Option<String>
    {
        self.order.clone()
    }


    /// Get the optional subtitle that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let subtitle = "subtitle".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .subtitle(Some(subtitle.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = item.subtitle();
    /// assert!(subtitle_opt.is_some());
    ///
    /// assert_eq!(subtitle.clone(), subtitle_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .subtitle(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = item.subtitle();
    /// assert!(subtitle_opt.is_none());
    /// ```
    fn subtitle(&self) -> Option<String>
    {
        self.subtitle.clone()
    }


    /// Get the optional summary that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let summary = "summary".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .summary(Some(summary.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = item.summary();
    /// assert!(summary_opt.is_some());
    ///
    /// assert_eq!(summary.clone(), summary_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .summary(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = item.summary();
    /// assert!(summary_opt.is_none());
    /// ```
    fn summary(&self) -> Option<String>
    {
        self.summary.clone()
    }


    /// Get the optional keywords that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let keywords = "keywords".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .keywords(Some(keywords.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = item.keywords();
    /// assert!(keywords_opt.is_some());
    ///
    /// assert_eq!(keywords.clone(), keywords_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesItemExtensionBuilder, ITunesItemExtensionGetters};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .keywords(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = item.keywords();
    /// assert!(keywords_opt.is_none());
    /// ```
    fn keywords(&self) -> Option<String>
    {
        self.keywords.clone()
    }
}
