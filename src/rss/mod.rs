// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
pub mod category;
pub mod cloud;
pub mod enclosure;
pub mod guid;
pub mod image;
pub mod item;
pub mod source;
pub mod text_input;

use chrono::*;
use rss::category::Category;
use rss::cloud::Cloud;
use rss::image::Image;
use rss::item::Item;
use rss::text_input::TextInput;
use util;

#[derive(Clone, Default)]
pub struct Channel {
    title:           String,
    link:            String,
    description:     String,
    language:        Option<String>,
    copyright:       Option<String>,
    managing_editor: Option<String>,
    web_master:      Option<String>,
    pub_date:        Option<DateTime<Local>>,
    last_build_date: Option<DateTime<Local>>,
    categories:      Option<Vec<Category>>,
    generator:       Option<String>,
    docs:            Option<String>,
    cloud:           Option<Cloud>,
    ttl:             Option<i64>,
    image:           Option<Image>,
    rating:          Option<String>,
    text_input:      Option<TextInput>,
    skip_hours:      Option<Vec<i64>>,
    skip_days:       Option<Vec<String>>,
    items:           Option<Vec<Item>>,
}


impl Channel {
    pub fn title(&self) -> String {
        self.title.clone()
    }


    pub fn link(&self) -> String {
        self.link.clone()
    }


    pub fn description(&self) -> String {
        self.description.clone()
    }


    pub fn language(&self) -> Option<String> {
        self.language.clone()
    }


    pub fn copyright(&self) -> Option<String> {
        self.copyright.clone()
    }


    pub fn managing_editor(&self) -> Option<String> {
        self.managing_editor.clone()
    }


    pub fn web_master(&self) -> Option<String> {
        self.web_master.clone()
    }


    pub fn pub_date(&self) -> Option<DateTime<Local>> {
        self.pub_date
    }


    pub fn last_build_date(&self) -> Option<DateTime<Local>> {
        self.last_build_date.clone()
    }


    pub fn categories(&self) -> Option<Vec<Category>> {
        self.categories.clone()
    }


    pub fn generator(&self) -> Option<String> {
        self.generator.clone()
    }


    pub fn docs(&self) -> Option<String> {
        self.docs.clone()
    }


    pub fn cloud(&self) -> Option<Cloud> {
        self.cloud.clone()
    }


    pub fn ttl(&self) -> Option<i64> {
        self.ttl.clone()
    }


    pub fn image(&self) -> Option<Image> {
        self.image.clone()
    }


    pub fn rating(&self) -> Option<String> {
        self.rating.clone()
    }


    pub fn text_input(&self) -> Option<TextInput> {
        self.text_input.clone()
    }


    pub fn skip_hours(&self) -> Option<Vec<i64>> {
        self.skip_hours.clone()
    }


    pub fn skip_days(&self) -> Option<Vec<String>> {
        self.skip_days.clone()
    }


    pub fn items(&self) -> Option<Vec<Item>> {
        self.items.clone()
    }
}


pub struct ChannelBuilder {
    title:           String,
    link:            String,
    description:     String,
    language:        Option<String>,
    copyright:       Option<String>,
    managing_editor: Option<String>,
    web_master:      Option<String>,
    pub_date:        Option<DateTime<Local>>,
    last_build_date: Option<DateTime<Local>>,
    categories:      Option<Vec<Category>>,
    generator:       Option<String>,
    docs:            Option<String>,
    cloud:           Option<Cloud>,
    ttl:             Option<i64>,
    image:           Option<Image>,
    rating:          Option<String>,
    text_input:      Option<TextInput>,
    skip_hours:      Option<Vec<i64>>,
    skip_days:       Option<Vec<String>>,
    items:           Option<Vec<Item>>,
}


impl ChannelBuilder {
    pub fn new() -> ChannelBuilder {
        ChannelBuilder {
            title:           String::new(),
            link:            String::new(),
            description:     String::new(),
            language:        None,
            copyright:       None,
            managing_editor: None,
            web_master:      None,
            pub_date:        None,
            last_build_date: None,
            categories:      None,
            generator:       None,
            docs:            None,
            cloud:           None,
            ttl:             None,
            image:           None,
            rating:          None,
            text_input:      None,
            skip_hours:      None,
            skip_days:       None,
            items:           None,
        }
    }


    pub fn title(&mut self, title: &str) -> &mut ChannelBuilder {
        self.title = title.to_string();
        self
    }


    pub fn link(&mut self, link: &str) -> &mut ChannelBuilder {
        self.link = link.to_string();
        self
    }


    pub fn description(&mut self, description: &str) -> &mut ChannelBuilder {
        self.description = description.to_string();
        self
    }


    pub fn language(&mut self, language: Option<String>) -> &mut ChannelBuilder {
        self.language = language;
        self
    }


    pub fn copyright(&mut self, copyright: Option<String>) -> &mut ChannelBuilder {
        self.copyright = copyright;
        self
    }


    pub fn managing_editor(&mut self, managing_editor: Option<String>) -> &mut ChannelBuilder {
        self.managing_editor = managing_editor;
        self
    }


    pub fn web_master(&mut self, web_master: Option<String>) -> &mut ChannelBuilder {
        self.web_master = web_master;
        self
    }


    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ChannelBuilder {
        if pub_date.is_some() {
            let pub_date_string = pub_date.unwrap();
            match Local.datetime_from_str(&pub_date_string, util::date_format()){
                Ok(date)    => {
                    self.pub_date = Some(date);
                },
                Err(err) => {
                    println!("Error: {}", err);
                },
            };
        }
        self
    }


    pub fn last_build_date(&mut self, last_build_date: Option<String>) -> &mut ChannelBuilder {
        if last_build_date.is_some() {
            let last_build_date_string = last_build_date.unwrap();
            match Local.datetime_from_str(&last_build_date_string, util::date_format()){
                Ok(date)    => {
                    self.last_build_date = Some(date);
                },
                Err(err) => {
                    println!("Error: {}", err);
                },
            };
        }
        self
    }


    pub fn categories(&mut self, categories: Option<Vec<Category>>) -> &mut ChannelBuilder {
        self.categories = categories;
        self
    }


    pub fn generator(&mut self, generator: Option<String>) -> &mut ChannelBuilder {
        self.generator = generator;
        self
    }


    pub fn docs(&mut self, docs: Option<String>) -> &mut ChannelBuilder {
        self.docs = docs;
        self
    }


    pub fn cloud(&mut self, cloud: Option<Cloud>) -> &mut ChannelBuilder {
        self.cloud = cloud;
        self
    }


    pub fn ttl(&mut self, ttl: Option<i64>) -> &mut ChannelBuilder {
        self.ttl = ttl;
        self
    }


    pub fn image(&mut self, image: Option<Image>) -> &mut ChannelBuilder {
        self.image = image;
        self
    }


    pub fn rating(&mut self, rating: Option<String>) -> &mut ChannelBuilder {
        self.rating = rating;
        self
    }


    pub fn text_input(&mut self, text_input: Option<TextInput>) -> &mut ChannelBuilder {
        self.text_input = text_input;
        self
    }


    pub fn skip_hours(&mut self, skip_hours: Option<Vec<i64>>) -> &mut ChannelBuilder {
        self.skip_hours = skip_hours;
        self
    }


    pub fn skip_days(&mut self, skip_days: Option<Vec<String>>) -> &mut ChannelBuilder {
        self.skip_days = skip_days;
        self
    }


    pub fn items(&mut self, items: Option<Vec<Item>>) -> &mut ChannelBuilder {
        self.items = items;
        self
    }


    pub fn finalize(&self) -> Channel {
        Channel {
            title:           self.title.clone(),
            link:            self.link.clone(),
            description:     self.description.clone(),
            language:        self.language.clone(),
            copyright:       self.copyright.clone(),
            managing_editor: self.managing_editor.clone(),
            web_master:      self.web_master.clone(),
            pub_date:        self.pub_date.clone(),
            last_build_date: self.last_build_date.clone(),
            categories:      self.categories.clone(),
            generator:       self.generator.clone(),
            docs:            self.docs.clone(),
            cloud:           self.cloud.clone(),
            ttl:             self.ttl.clone(),
            image:           self.image.clone(),
            rating:          self.rating.clone(),
            text_input:      self.text_input.clone(),
            skip_hours:      self.skip_hours.clone(),
            skip_days:       self.skip_days.clone(),
            items:           self.items.clone(),
        }
    }
}
