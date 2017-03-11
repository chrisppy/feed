// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

//! The fields can be set for channels by using the methods under
//! `ChannelBuilder`.


use channels::{Category, Channel, ChannelBuilder, Cloud, Image, Item, TextInput};
use enums::Day;
use utils::string_utils;


impl ChannelBuilder
{
    /// Construct a new `ChannelBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channel_builder = ChannelBuilder::new();
    /// ```
    pub fn new() -> ChannelBuilder
    {
        ChannelBuilder::default()
    }


    /// Set the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.title("The Linux Action Show! OGG");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut ChannelBuilder
    {
        self.title = title.to_owned();
        self
    }


    /// Set the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.link("http://www.jupiterbroadcasting.com");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut ChannelBuilder
    {
        self.link = link.to_owned();
        self
    }


    /// Set the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.description(description.as_ref());
    /// ```
    pub fn description(&mut self, description: &str) -> &mut ChannelBuilder
    {
        self.description = description.to_owned();
        self
    }


    /// Set the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.language(Some("en".to_owned()));
    /// ```
    pub fn language(&mut self, language: Option<String>) -> &mut ChannelBuilder
    {
        self.language = language;
        self
    }


    /// Set the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let copyright = "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.copyright(Some(copyright));
    /// ```
    pub fn copyright(&mut self, copyright: Option<String>) -> &mut ChannelBuilder
    {
        self.copyright = copyright;
        self
    }


    /// Set the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let managing_editor =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.managing_editor(Some(managing_editor));
    /// ```
    pub fn managing_editor(&mut self, managing_editor: Option<String>) -> &mut ChannelBuilder
    {
        self.managing_editor = managing_editor;
        self
    }


    /// Set the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let web_master =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.web_master(Some(web_master));
    /// ```
    pub fn web_master(&mut self, web_master: Option<String>) -> &mut ChannelBuilder
    {
        self.web_master = web_master;
        self
    }


    /// Set the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.pub_date(Some("Sun, 13 Mar 2016 20:02:02
    /// -0700".to_owned()));
    /// ```
    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ChannelBuilder
    {
        self.pub_date = pub_date;
        self
    }


    /// Set the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.last_build_date(Some("Sun, 13 Mar 2016 20:02:02
    /// -0700".to_owned()));
    /// ```
    pub fn last_build_date(&mut self, last_build_date: Option<String>) -> &mut ChannelBuilder
    {
        self.last_build_date = last_build_date;
        self
    }


    /// Set the optional categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, CategoryBuilder};
    ///
    /// let category = CategoryBuilder::new().finalize();
    /// let categories = vec![category];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.categories(Some(categories));
    /// ```
    pub fn categories(&mut self, categories: Option<Vec<Category>>) -> &mut ChannelBuilder
    {
        self.categories = categories;
        self
    }


    /// Set the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let generator = "Feeder 2.5.12(2294); ".to_owned()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.generator(Some(generator));
    /// ```
    pub fn generator(&mut self, generator: Option<String>) -> &mut ChannelBuilder
    {
        self.generator = generator;
        self
    }


    /// Set the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.docs(Some("http://blogs.law.harvard.edu/tech/rss/".
    /// to_owned()));
    /// ```
    pub fn docs(&mut self, docs: Option<String>) -> &mut ChannelBuilder
    {
        self.docs = docs;
        self
    }


    /// Set the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.cloud(Some(cloud));
    /// ```
    pub fn cloud(&mut self, cloud: Option<Cloud>) -> &mut ChannelBuilder
    {
        self.cloud = cloud;
        self
    }


    /// Set the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.ttl(Some(60));
    /// ```
    pub fn ttl(&mut self, ttl: Option<i64>) -> &mut ChannelBuilder
    {
        self.ttl = ttl;
        self
    }


    /// Set the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, ImageBuilder};
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.image(Some(image));
    /// ```
    pub fn image(&mut self, image: Option<Image>) -> &mut ChannelBuilder
    {
        self.image = image;
        self
    }

    /// Set the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.rating(Some("PG-13".to_owned()));
    /// ```
    pub fn rating(&mut self, rating: Option<String>) -> &mut ChannelBuilder
    {
        self.rating = rating;
        self
    }


    /// Set the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .link("http://www.example.com/feedback")
    ///     .finalize();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.text_input(Some(text_input));
    /// ```
    pub fn text_input(&mut self, text_input: Option<TextInput>) -> &mut ChannelBuilder
    {
        self.text_input = text_input;
        self
    }


    /// Set the optional skipdays that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let hours: Vec<i64> = vec![0, 12, 18];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_hours(Some(hours));
    /// ```
    pub fn skip_hours(&mut self, skip_hours: Option<Vec<i64>>) -> &mut ChannelBuilder
    {
        self.skip_hours = skip_hours;
        self
    }


    /// Set the optional skipdays that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let days = vec!["Monday".to_owned(), "Tuesday".to_owned()];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_days(Some(days));
    /// ```
    pub fn skip_days(&mut self, skip_days: Option<Vec<String>>) -> &mut ChannelBuilder
    {
        self.skip_days = skip_days;
        self
    }


    /// Set the optional items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, ItemBuilder};
    ///
    /// let title = "Making Music with Linux | LAS 408".to_owned();
    ///
    /// let item = ItemBuilder::new().title(Some(title)).finalize();
    /// let items = vec![item];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.items(Some(items));
    /// ```
    pub fn items(&mut self, items: Option<Vec<Item>>) -> &mut ChannelBuilder
    {
        self.items = items;
        self
    }


    /// Construct the `Channel` from the `ChannelBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channels = ChannelBuilder::new()
    ///         .title("The Linux Action Show! OGG")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .description(description.as_ref())
    ///         .language(None)
    ///         .copyright(None)
    ///         .managing_editor(None)
    ///         .web_master(None)
    ///         .pub_date(None)
    ///         .last_build_date(None)
    ///         .categories(None)
    ///         .generator(None)
    ///         .docs(None)
    ///         .cloud(None)
    ///         .ttl(None)
    ///         .image(None)
    ///         .rating(None)
    ///         .text_input(None)
    ///         .skip_hours(None)
    ///         .skip_days(None)
    ///         .items(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Result<Channel, String>
    {
        let link = string_utils::str_to_url(self.link.as_str())?;
        let pub_date = string_utils::option_string_to_option_date(self.pub_date.clone())?;
        let last_build_date = string_utils::option_string_to_option_date(self.last_build_date.clone())?;

        let docs = match self.docs.clone()
        {
            Some(val) => Some(string_utils::str_to_url(val.as_str())?),
            None => None,
        };

        let skip_days = match self.skip_days.clone()
        {
            Some(val) =>
            {
                let mut days = Vec::new();

                let mut skip_days_vec = val;
                skip_days_vec.sort();
                skip_days_vec.dedup();
                for day in skip_days_vec
                {
                    days.push(Day::value_of(day.as_str())?);
                }

                Some(days)
            }
            None => None,
        };

        if self.ttl.is_some() && self.ttl.unwrap() < 0
        {
            return Err("Channel ttl cannot be a negative value.".to_owned());
        }

        if self.skip_hours.is_some()
        {
            let mut skip_hours_vec = self.skip_hours.clone().unwrap();
            skip_hours_vec.sort();
            skip_hours_vec.dedup();
            for hour in skip_hours_vec
            {
                if hour < 0
                {
                    return Err("Channel Skip Hour cannot be a negative value.".to_owned());
                }
                else if hour > 23
                {
                    return Err("Channel Skip Hour cannot be greater than 23.".to_owned());
                }
            }
        }

        Ok(Channel {
               title: self.title.clone(),
               link: link,
               description: self.description.clone(),
               language: self.language.clone(),
               copyright: self.copyright.clone(),
               managing_editor: self.managing_editor.clone(),
               web_master: self.web_master.clone(),
               pub_date: pub_date,
               last_build_date: last_build_date,
               categories: self.categories.clone(),
               generator: self.generator.clone(),
               docs: docs,
               cloud: self.cloud.clone(),
               ttl: self.ttl,
               image: self.image.clone(),
               rating: self.rating.clone(),
               text_input: self.text_input.clone(),
               skip_hours: self.skip_hours.clone(),
               skip_days: skip_days,
               items: self.items.clone(),
           })
    }
}
