// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields can be set for channel by using the methods under `ChannelBuilder`.

use errors;
use rss::{Category, Channel, ChannelBuilder, Cloud, Image, Item, TextInput};
use url::Url;
use util;

impl ChannelBuilder {
    /// Construct a new `ChannelBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel_builder = ChannelBuilder::new();
    /// ```
    pub fn new() -> ChannelBuilder {
        ChannelBuilder::default()
    }


    /// Set the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.title("The Linux Action Show! OGG");
    /// ```
    pub fn title(&mut self, title: &str) -> &mut ChannelBuilder {
        self.title = title.to_owned();
        self
    }


    /// Set the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.link("http://www.jupiterbroadcasting.com");
    /// ```
    pub fn link(&mut self, link: &str) -> &mut ChannelBuilder {
        self.link = link.to_owned();
        self
    }


    /// Set the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.description(description.as_ref());
    /// ```
    pub fn description(&mut self, description: &str) -> &mut ChannelBuilder {
        self.description = description.to_owned();
        self
    }


    /// Set the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.language(Some("en".to_owned()));
    /// ```
    pub fn language(&mut self, language: Option<String>) -> &mut ChannelBuilder {
        self.language = language;
        self
    }


    /// Set the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let copyright = "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.copyright(Some(copyright));
    /// ```
    pub fn copyright(&mut self, copyright: Option<String>) -> &mut ChannelBuilder {
        self.copyright = copyright;
        self
    }


    /// Set the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let managing_editor =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.managing_editor(Some(managing_editor));
    /// ```
    pub fn managing_editor(&mut self, managing_editor: Option<String>) -> &mut ChannelBuilder {
        self.managing_editor = managing_editor;
        self
    }


    /// Set the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let web_master =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.web_master(Some(web_master));
    /// ```
    pub fn web_master(&mut self, web_master: Option<String>) -> &mut ChannelBuilder {
        self.web_master = web_master;
        self
    }


    /// Set the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.pub_date(Some("Sun, 13 Mar 2016 20:02:02 -0700".to_owned()));
    /// ```
    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ChannelBuilder {
        self.pub_date = util::option_string_to_option_date(pub_date);
        self
    }


    /// Set the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.last_build_date(Some("Sun, 13 Mar 2016 20:02:02 -0700".to_owned()));
    /// ```
    pub fn last_build_date(&mut self, last_build_date: Option<String>) -> &mut ChannelBuilder {
        self.last_build_date = util::option_string_to_option_date(last_build_date);
        self
    }


    /// Set the optional categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{ChannelBuilder, CategoryBuilder};
    ///
    /// let category = CategoryBuilder::new()
    ///     .category("Title")
    ///     .finalize();
    /// let categories = vec![category];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.categories(Some(categories));
    /// ```
    pub fn categories(&mut self, categories: Option<Vec<Category>>) -> &mut ChannelBuilder {
        self.categories = categories;
        self
    }


    /// Set the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let generator = "Feeder 2.5.12(2294); ".to_owned()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.generator(Some(generator));
    /// ```
    pub fn generator(&mut self, generator: Option<String>) -> &mut ChannelBuilder {
        self.generator = generator;
        self
    }


    /// Set the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.docs(Some("http://blogs.law.harvard.edu/tech/rss".to_owned()));
    /// ```
    pub fn docs(&mut self, docs: Option<String>) -> &mut ChannelBuilder {
        self.docs = docs;
        self
    }


    /// Set the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{ChannelBuilder, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("http://rpc.sys.com/")
    ///     .path("/RPC2")
    ///     .register_procedure("pingMe")
    ///     .protocol("soap")
    ///     .finalize();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.cloud(Some(cloud));
    /// ```
    pub fn cloud(&mut self, cloud: Option<Cloud>) -> &mut ChannelBuilder {
        self.cloud = cloud;
        self
    }


    /// Set the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.ttl(Some(60));
    /// ```
    pub fn ttl(&mut self, ttl: Option<i64>) -> &mut ChannelBuilder {
        if ttl.is_some() {
            let ttl_num = ttl.unwrap();
            if ttl_num < 0 {
                panic!(errors::negative_error("ttl", ttl_num));
            }
        }
        self.ttl = ttl;
        self
    }


    /// Set the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{ChannelBuilder, ImageBuilder};
    ///
    /// let image = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .link("http://jupiterbroadcasting.com/")
    ///     .finalize();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.image(Some(image));
    /// ```
    pub fn image(&mut self, image: Option<Image>) -> &mut ChannelBuilder {
        self.image = image;
        self
    }

    /// Set the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.rating(Some("PG-13".to_owned()));
    /// ```
    pub fn rating(&mut self, rating: Option<String>) -> &mut ChannelBuilder {
        self.rating = rating;
        self
    }


    /// Set the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{ChannelBuilder, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .title("Enter Comment")
    ///     .description("Provided Feedback")
    ///     .name("Comment")
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.text_input(Some(text_input));
    /// ```
    pub fn text_input(&mut self, text_input: Option<TextInput>) -> &mut ChannelBuilder {
        self.text_input = text_input;
        self
    }


    /// Set the optional skipdays that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let hours: Vec<i64> = vec![0, 12, 18];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_hours(Some(hours));
    /// ```
    pub fn skip_hours(&mut self, skip_hours: Option<Vec<i64>>) -> &mut ChannelBuilder {
        if skip_hours.is_some() {
            let mut skip_hours_vec = skip_hours.clone().unwrap();
            skip_hours_vec.sort();
            skip_hours_vec.dedup();
            for hour in skip_hours_vec {
                if hour < 0 {
                    panic!(errors::negative_error("skip hours", hour));
                } else if hour > 23 {
                    panic!(errors::invalid_int_error("skip hours", hour));
                }
            }
        }
        self.skip_hours = skip_hours.clone();
        self
    }


    /// Set the optional skipdays that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let days = vec!["Monday".to_owned(), "Tuesday".to_owned()];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_days(Some(days));
    /// ```
    pub fn skip_days(&mut self, skip_days: Option<Vec<String>>) -> &mut ChannelBuilder {
        if skip_days.is_some() {
            let mut skip_days_vec = skip_days.clone().unwrap();
            skip_days_vec.sort();
            skip_days_vec.dedup();
            for day in skip_days_vec {
                let day_upper = day.to_uppercase();
                match day_upper.as_str() {
                    "MONDAY" => (),
                    "TUESDAY" => (),
                    "WEDNESDAY" => (),
                    "THURSDAY" => (),
                    "FRIDAY" => (),
                    "SATURDAY" => (),
                    "SUNDAY" => (),
                    _ => panic!(errors::invalid_str_error("skip days", day.as_str())),
                }
            }
        }
        self.skip_days = skip_days.clone();
        self
    }


    /// Set the optional items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{ChannelBuilder, ItemBuilder};
    ///
    /// let title = "Making Music with Linux | LAS 408".to_owned();
    ///
    /// let item = ItemBuilder::new().title(Some(title)).finalize();
    /// let items = vec![item];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.items(Some(items));
    /// ```
    pub fn items(&mut self, items: Option<Vec<Item>>) -> &mut ChannelBuilder {
        self.items = items;
        self
    }


    /// Construct the `Channel` from the `ChannelBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channel = ChannelBuilder::new()
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
    pub fn finalize(&self) -> Channel {
        if self.title.is_empty() {
            panic!(errors::empty_string_error("Channel title"));
        }
        if self.link.is_empty() {
            panic!(errors::empty_string_error("Channel link"));
        }
        if self.description.is_empty() {
            panic!(errors::empty_string_error("Channel description"));
        }

        let link_str = self.link.clone();
        let link = Url::parse(link_str.as_str())
            .expect(errors::url_parse_error(link_str.as_str()).as_str());

        Channel {
            title: self.title.clone(),
            link: link,
            description: self.description.clone(),
            language: self.language.clone(),
            copyright: self.copyright.clone(),
            managing_editor: self.managing_editor.clone(),
            web_master: self.web_master.clone(),
            pub_date: self.pub_date,
            last_build_date: self.last_build_date,
            categories: self.categories.clone(),
            generator: self.generator.clone(),
            docs: self.docs.clone(),
            cloud: self.cloud.clone(),
            ttl: self.ttl,
            image: self.image.clone(),
            rating: self.rating.clone(),
            text_input: self.text_input.clone(),
            skip_hours: self.skip_hours.clone(),
            skip_days: self.skip_days.clone(),
            items: self.items.clone(),
        }
    }
}
