// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! `Validate` Trait for `Channel`


use CategoryBuilder;
use CategoryGetters;
use ChannelBuilder;
use ChannelGetters;
use CloudBuilder;
use CloudGetters;
use EnclosureBuilder;
use EnclosureGetters;
use GuidBuilder;
use GuidGetters;
use ImageBuilder;
use ImageGetters;
use ItemBuilder;
use ItemGetters;
use SourceBuilder;
use SourceGetters;
use TextInputBuilder;
use TextInputGetters;
use Validate;
use extension::itunes::{ITunesCategoryBuilder, ITunesCategoryGetters, ITunesChannelExtensionBuilder,
                        ITunesChannelExtensionGetters, ITunesItemExtensionBuilder, ITunesItemExtensionGetters,
                        ITunesOwnerBuilder, ITunesOwnerGetters};
use rss::{Category, Channel, Item};
use rss::extension::itunes::ITunesCategory;
use utils::string_utils;

impl Validate for Channel
{
    /// Validate `Channel`
    ///
    /// ## Examples
    /// ```
    /// extern crate rss;
    /// extern crate feed;
    ///
    /// use feed::{FromUrl, ChannelGetters, Validate};
    /// use rss::Channel;
    ///
    /// fn main()
    /// {
    ///     let url = "https://feedpress.me/usererror.xml";
    ///
    ///     let channel = Channel::from_url(url).unwrap();
    ///     channel.validate().unwrap();
    /// }
    /// ```
    fn validate(&self) -> Result<Channel, String>
    {
        let cloud = match self.cloud()
        {
            None => None,
            Some(val) =>
            {
                Some(CloudBuilder::new()
                         .domain(val.domain().as_str())
                         .port(string_utils::string_to_i64(val.port().as_str())?)
                         .path(val.path().as_str())
                         .register_procedure(val.register_procedure().as_str())
                         .protocol(val.protocol().as_str())
                         .validate()?
                         .finalize()?)
            }
        };

        let mut channel_cat: Vec<Category> = Vec::new();
        for cat in self.categories()
        {
            channel_cat.push(CategoryBuilder::new()
                                 .name(cat.name().as_str())
                                 .domain(cat.domain())
                                 .validate()?
                                 .finalize()?);
        }

        let mut skip_hours: Vec<i64> = Vec::new();
        for hour in self.skip_hours()
        {
            skip_hours.push(string_utils::string_to_i64(hour.as_str())?);
        }

        let image = match self.image()
        {
            None => None,
            Some(val) =>
            {
                Some(ImageBuilder::new()
                         .url(val.url().as_str())
                         .title(val.title().as_str())
                         .link(val.link().as_str())
                         .width(string_utils::option_string_to_option_i64(val.width())?)
                         .height(string_utils::option_string_to_option_i64(val.height())?)
                         .description(val.description())
                         .validate()?
                         .finalize()?)
            }
        };

        let text_input = match self.text_input()
        {
            None => None,
            Some(val) =>
            {
                Some(TextInputBuilder::new()
                         .title(val.title().as_str())
                         .description(val.description().as_str())
                         .name(val.name().as_str())
                         .link(val.link().as_str())
                         .validate()?
                         .finalize()?)
            }
        };

        let mut items: Vec<Item> = Vec::new();
        for item in self.items()
        {
            let mut item_cat: Vec<Category> = Vec::new();
            for cat in item.categories()
            {
                item_cat.push(CategoryBuilder::new()
                                  .name(cat.name().as_str())
                                  .domain(cat.domain())
                                  .validate()?
                                  .finalize()?);
            }

            let enclosure = match item.enclosure()
            {
                None => None,
                Some(eval) =>
                {
                    Some(EnclosureBuilder::new()
                             .url(eval.url().as_str())
                             .length(string_utils::string_to_i64(eval.length.as_str())?)
                             .mime_type(eval.mime_type().as_str())
                             .validate()?
                             .finalize()?)
                }
            };

            let guid = match item.guid()
            {
                None => None,
                Some(gval) =>
                {
                    Some(GuidBuilder::new()
                             .value(gval.value().as_str())
                             .is_permalink(Some(gval.is_permalink()))
                             .finalize()?)
                }
            };

            let source = match item.source()
            {
                None => None,
                Some(sval) =>
                {
                    Some(SourceBuilder::new()
                             .url(sval.url().as_str())
                             .title(sval.title())
                             .validate()?
                             .finalize()?)
                }
            };

            let itunes_item = match item.itunes_ext()
            {
                None => None,
                Some(ival) =>
                {
                    Some(ITunesItemExtensionBuilder::new()
                             .author(ival.author())
                             .block(ival.block())
                             .image(ival.image())
                             .duration(ival.duration())
                             .explicit(ival.explicit())
                             .closed_captioned(ival.closed_captioned())
                             .order(ival.order())
                             .subtitle(ival.subtitle())
                             .summary(ival.summary())
                             .keywords(ival.keywords())
                             .finalize()?)
                }
            };

            items.push(ItemBuilder::new()
                           .title(item.title())
                           .link(item.link())
                           .description(item.description())
                           .author(item.author())
                           .pub_date(item.pub_date())
                           .comments(item.comments())
                           .categories(item_cat)
                           .enclosure(enclosure)
                           .guid(guid)
                           .source(source)
                           .itunes_ext(itunes_item)
                           .validate()?
                           .finalize()?);
        }

        let itunes_channel = match self.itunes_ext()
        {
            None => None,
            Some(cval) =>
            {
                let itunes_owner = match cval.owner()
                {
                    None => None,
                    Some(oval) =>
                    {
                        Some(ITunesOwnerBuilder::new()
                                 .name(oval.name())
                                 .email(oval.email())
                                 .finalize()?)
                    }
                };

                let mut itunes_cat: Vec<ITunesCategory> = Vec::new();
                for cat in cval.categories()
                {
                    itunes_cat.push(ITunesCategoryBuilder::new()
                                        .text(cat.text().as_str())
                                        .subcategory(cat.subcategory())
                                        .finalize()?);
                }

                Some(ITunesChannelExtensionBuilder::new()
                         .author(cval.author())
                         .block(cval.block())
                         .image(cval.image())
                         .explicit(cval.explicit())
                         .complete(cval.complete())
                         .new_feed_url(cval.new_feed_url())
                         .subtitle(cval.subtitle())
                         .summary(cval.summary())
                         .keywords(cval.keywords())
                         .categories(itunes_cat)
                         .owner(itunes_owner)
                         .finalize()?)
            }
        };

        ChannelBuilder::new()
            .title(self.title().as_str())
            .link(self.link().as_str())
            .description(self.description().as_str())
            .language(self.language())
            .copyright(self.copyright())
            .managing_editor(self.managing_editor())
            .webmaster(self.webmaster())
            .pub_date(self.pub_date())
            .last_build_date(self.last_build_date())
            .generator(self.generator())
            .docs(self.docs())
            .rating(None)
            .ttl(string_utils::option_string_to_option_i64(self.ttl())?)
            .cloud(cloud)
            .categories(channel_cat)
            .image(image)
            .text_input(text_input)
            .skip_hours(skip_hours)
            .skip_days(self.skip_days())
            .items(items)
            .itunes_ext(itunes_channel)
            .validate()?
            .finalize()
    }
}
