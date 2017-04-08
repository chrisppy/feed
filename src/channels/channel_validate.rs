// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! `Validate` Trait for `Channel`


use channels::{CategoryBuilder, CategoryGetters, ChannelBuilder, ChannelGetters, CloudBuilder, CloudGetters,
               ImageBuilder, ImageGetters, Validate};
use rss::{Category, Channel};
use utils::string_utils;

impl Validate for Channel
{
    /// Validate `Channel`
    fn validate(&self) -> Result<Channel, String>
    {
        let cloud = match self.cloud()
        {
            None => None,
            Some(val) =>
            {
                Some(CloudBuilder::new().domain(val.domain().as_str())
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
            channel_cat.push(CategoryBuilder::new().name(cat.name().as_str())
                                 .domain(cat.domain())
                                 .validate()?
                                 .finalize()?);
        }

        let channel_cat_opt = if channel_cat.is_empty()
        {
            None
        }
        else
        {
            Some(channel_cat)
        };


        let image = match self.image()
        {
            None => None,
            Some(val) =>
            {
                Some(ImageBuilder::new().url(val.url().as_str())
                         .title(val.title().as_str())
                         .link(val.link().as_str())
                         .width(string_utils::option_string_to_option_i64(val.width())?)
                         .height(string_utils::option_string_to_option_i64(val.height())?)
                         .description(val.description())
                         .validate()?
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
            .categories(channel_cat_opt)
            .image(image)
            .text_input(None)
            .skip_hours(None)
            .skip_days(None)
            .items(None)
            .itunes_ext(None)
            .validate()?
            .finalize()
    }
}
