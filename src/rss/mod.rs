// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

//! The fields under channel can be retrieved by using the methods under `Channel`
//! and the fields can be set for channel by using the methods under `ChannelBuilder`.

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

/// This `Channel` struct contains all the items that exist for the `feed`.
#[derive(Clone, Default)]
pub struct Channel {
    title:           String,
    link:            String,
    description:     String,
    language:        Option<String>,
    copyright:       Option<String>,
    managing_editor: Option<String>,
    web_master:      Option<String>,
    pub_date:        Option<DateTime<FixedOffset>>,
    last_build_date: Option<DateTime<FixedOffset>>,
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
    /// Get the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let title = "The Linux Action Show! OGG";
    /// let channel = ChannelBuilder::new()
    ///     .title(title)
    ///     .finalize();
    /// assert_eq!(title.to_owned(), channel.title());
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }


    /// Get the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    /// let channel = ChannelBuilder::new()
    ///     .link(link)
    ///     .finalize();
    /// assert_eq!(link.to_owned(), channel.link());
    /// ```
    pub fn link(&self) -> String {
        self.link.clone()
    }


    /// Get the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!";
    /// let channel = ChannelBuilder::new()
    ///     .description(description)
    ///     .finalize();
    /// assert_eq!(description.to_owned(), channel.description());
    /// ```
    pub fn description(&self) -> String {
        self.description.clone()
    }


    /// Get the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let language_string = "en".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .language(Some(language_string.clone()))
    ///     .finalize();
    /// let language_option = channel.language();
    /// assert!(language_option.is_some());
    /// let language = language_option.unwrap();
    /// assert_eq!(language_string.clone(), language);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .language(None)
    ///     .finalize();
    /// assert!(channel.language().is_none());
    /// ```
    pub fn language(&self) -> Option<String> {
        self.language.clone()
    }


    /// Get the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let copyright_string = "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .copyright(Some(copyright_string.clone()))
    ///     .finalize();
    /// let copyright_option = channel.copyright();
    /// assert!(copyright_option.is_some());
    /// let copyright = copyright_option.unwrap();
    /// assert_eq!(copyright_string.clone(), copyright);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .copyright(None)
    ///     .finalize();
    /// assert!(channel.copyright().is_none());
    /// ```
    pub fn copyright(&self) -> Option<String> {
        self.copyright.clone()
    }


    /// Get the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let managing_editor_string = "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .managing_editor(Some(managing_editor_string.clone()))
    ///     .finalize();
    /// let managing_editor_option = channel.managing_editor();
    /// assert!(managing_editor_option.is_some());
    /// let managing_editor = managing_editor_option.unwrap();
    /// assert_eq!(managing_editor_string.clone(), managing_editor);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .managing_editor(None)
    ///     .finalize();
    /// assert!(channel.managing_editor().is_none());
    /// ```
    pub fn managing_editor(&self) -> Option<String> {
        self.managing_editor.clone()
    }

    /// Get the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let web_master_string = "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .web_master(Some(web_master_string.clone()))
    ///     .finalize();
    /// let web_master_option = channel.web_master();
    /// assert!(web_master_option.is_some());
    /// let web_master = web_master_option.unwrap();
    /// assert_eq!(web_master_string.clone(), web_master);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .web_master(None)
    ///     .finalize();
    /// assert!(channel.web_master().is_none());
    /// ```
    pub fn web_master(&self) -> Option<String> {
        self.web_master.clone()
    }


    /// Get the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    /// let channel = ChannelBuilder::new()
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .finalize();
    /// let local = channel.pub_date();
    /// assert!(local.is_some());
    /// let local_result = local.unwrap();
    /// assert_eq!(pub_date.to_owned(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .pub_date(None)
    ///     .finalize();
    /// assert!(channel.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<DateTime<FixedOffset>> {
        self.pub_date
    }


    /// Get the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    /// let channel = ChannelBuilder::new()
    ///     .last_build_date(Some(last_build_date.to_owned()))
    ///     .finalize();
    /// let local = channel.last_build_date();
    /// assert!(local.is_some());
    /// let local_result = local.unwrap();
    /// assert_eq!(last_build_date.to_owned(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .last_build_date(None)
    ///     .finalize();
    /// assert!(channel.last_build_date().is_none());
    /// ```
    pub fn last_build_date(&self) -> Option<DateTime<FixedOffset>> {
        self.last_build_date
    }


    /// Get the optional categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category_1 = CategoryBuilder::new()
    ///     .domain(None)
    ///     .category("Media")
    ///     .finalize();
    /// let category_2 = CategoryBuilder::new()
    ///     .domain(Some("http://jupiterbroadcasting.com".to_owned()))
    ///     .category("Podcast")
    ///     .finalize();
    /// let categories_vec = vec![category_1, category_2];
    /// let channel = ChannelBuilder::new()
    ///     .categories(Some(categories_vec.clone()))
    ///     .finalize();
    /// let categories_option = channel.categories();
    /// assert!(categories_option.is_some());
    /// let categories = categories_option.unwrap();
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .categories(None)
    ///     .finalize();
    /// assert!(channel.categories().is_none());
    /// ```
    pub fn categories(&self) -> Option<Vec<Category>> {
        self.categories.clone()
    }


    /// Get the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let generator_string = "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .generator(Some(generator_string.clone()))
    ///     .finalize();
    /// let generator_option = channel.generator();
    /// assert!(generator_option.is_some());
    /// let generator = generator_option.unwrap();
    /// assert_eq!(generator_string.clone(), generator);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .generator(None)
    ///     .finalize();
    /// assert!(channel.generator().is_none());
    /// ```
    pub fn generator(&self) -> Option<String> {
        self.generator.clone()
    }


    /// Get the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let docs_string = "http://blogs.law.harvard.edu/tech/rss".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .docs(Some(docs_string.clone()))
    ///     .finalize();
    /// let docs_option = channel.docs();
    /// assert!(docs_option.is_some());
    /// let docs = docs_option.unwrap();
    /// assert_eq!(docs_string.clone(), docs);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .docs(None)
    ///     .finalize();
    /// assert!(channel.docs().is_none());
    /// ```
    pub fn docs(&self) -> Option<String> {
        self.docs.clone()
    }

    /// Get the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("rpc.sys.com")
    ///     .port(80)
    ///     .path("/RPC2")
    ///     .register_procedure("pingMe")
    ///     .protocol("soap")
    ///     .finalize();
    /// let channel = ChannelBuilder::new()
    ///     .cloud(Some(cloud))
    ///     .finalize();
    /// assert!(channel.cloud().is_some());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .cloud(None)
    ///     .finalize();
    /// assert!(channel.cloud().is_none());
    /// ```
    pub fn cloud(&self) -> Option<Cloud> {
        self.cloud.clone()
    }


    /// Get the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let ttl_num = 60;
    /// let channel = ChannelBuilder::new()
    ///     .ttl(Some(ttl_num))
    ///     .finalize();
    /// let ttl_option = channel.ttl();
    /// assert!(ttl_option.is_some());
    /// let ttl = ttl_option.unwrap();
    /// assert_eq!(ttl_num, ttl);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .ttl(None)
    ///     .finalize();
    /// assert!(channel.ttl().is_none());
    /// ```
    pub fn ttl(&self) -> Option<i64> {
        self.ttl
    }


    /// Get the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com")
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .height(None)
    ///     .width(None)
    ///     .description(None)
    ///     .finalize();
    /// let channel = ChannelBuilder::new()
    ///     .image(Some(image))
    ///     .finalize();
    /// assert!(channel.image().is_some());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .image(None)
    ///     .finalize();
    /// assert!(channel.image().is_none());
    /// ```
    pub fn image(&self) -> Option<Image> {
        self.image.clone()
    }


    /// Get the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let rating_string = "PG-13:".to_owned();
    /// let channel = ChannelBuilder::new()
    ///     .rating(Some(rating_string.clone()))
    ///     .finalize();
    /// let rating_option = channel.rating();
    /// assert!(rating_option.is_some());
    /// let rating = rating_option.unwrap();
    /// assert_eq!(rating_string, rating);
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .rating(None)
    ///     .finalize();
    /// assert!(channel.rating().is_none());
    /// ```
    pub fn rating(&self) -> Option<String> {
        self.rating.clone()
    }


    /// Get the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .title("Enter Comment")
    ///     .description("Provided Feedback")
    ///     .name("Comment")
    ///     .link("www.example.com/feedback")
    ///     .finalize();
    /// let channel = ChannelBuilder::new()
    ///     .text_input(Some(text_input))
    ///     .finalize();
    /// assert!(channel.text_input().is_some());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .text_input(None)
    ///     .finalize();
    /// assert!(channel.text_input().is_none());
    /// ```
    pub fn text_input(&self) -> Option<TextInput> {
        self.text_input.clone()
    }

    /// Get the optional skip hours that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let skip_hours_vec: Vec<i64> = vec![6,7,8,14,22];
    /// let channel = ChannelBuilder::new()
    ///     .skip_hours(Some(skip_hours_vec.clone()))
    ///     .finalize();
    /// let skip_hours_option = channel.skip_hours();
    /// assert!(skip_hours_option.is_some());
    /// let skip_hours = skip_hours_option.unwrap();
    /// let len = skip_hours_vec.clone().len();
    /// assert_eq!(len, skip_hours.len());
    /// for x in 0..len {
    ///     assert_eq!(skip_hours_vec[x], skip_hours[x]);
    /// }
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .skip_hours(None)
    ///     .finalize();
    /// assert!(channel.skip_hours().is_none());
    /// ```
    pub fn skip_hours(&self) -> Option<Vec<i64>> {
        self.skip_hours.clone()
    }


    /// Get the optional skip days that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let skip_days_vec: Vec<String> = vec!["Monday".to_owned(),"Wednesday".to_owned(),"Thursday".to_owned(),"Sunday".to_owned()];
    /// let channel = ChannelBuilder::new()
    ///     .skip_days(Some(skip_days_vec.clone()))
    ///     .finalize();
    /// let skip_days_option = channel.skip_days();
    /// assert!(skip_days_option.is_some());
    /// let skip_days = skip_days_option.unwrap();
    /// let len = skip_days_vec.clone().len();
    /// assert_eq!(len, skip_days.len());
    /// for x in 0..len {
    ///     assert_eq!(skip_days_vec[x], skip_days[x]);
    /// }
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .skip_days(None)
    ///     .finalize();
    /// assert!(channel.skip_days().is_none());
    /// ```
    pub fn skip_days(&self) -> Option<Vec<String>> {
        self.skip_days.clone()
    }


    /// Get the optional items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item_1 = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some("http://www.jupiterbroadcasting.com/97561/making-music-with-linux-las-408/".to_owned()))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize();
    /// let item_2 = ItemBuilder::new()
    ///     .title(None)
    ///     .link(None)
    ///     .description(Some("<![CDATA[<p>In special Rasberry Pi 3 edition of the show we look at the new hardware, review & chat with Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for development of their open artificial intelligence platform & get the latest news.</p><p>Plus replacing Spotify on Linux, the new Microsoft lock-in, our hosts face a moral quandary & more!</p>]]>".to_owned()))
    ///     .author(None)
    ///     .categories(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize();
    /// let items_vec = vec![item_1, item_2];
    /// let channel = ChannelBuilder::new()
    ///     .items(Some(items_vec.clone()))
    ///     .finalize();
    /// let items_option = channel.items();
    /// assert!(items_option.is_some());
    /// let items = items_option.unwrap();
    /// assert_eq!(items_vec.clone().len(), items.len());
    /// ```
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::new()
    ///     .items(None)
    ///     .finalize();
    /// assert!(channel.items().is_none());
    /// ```
    pub fn items(&self) -> Option<Vec<Item>> {
        self.items.clone()
    }
}


/// This `ChannelBuilder` struct creates the `Channel`.
#[derive(Default)]
pub struct ChannelBuilder {
    title:           String,
    link:            String,
    description:     String,
    language:        Option<String>,
    copyright:       Option<String>,
    managing_editor: Option<String>,
    web_master:      Option<String>,
    pub_date:        Option<DateTime<FixedOffset>>,
    last_build_date: Option<DateTime<FixedOffset>>,
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
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.description("Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!");
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
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.copyright(Some("Copyright 2002, Spartanburg Herald-Journal".to_owned()));
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
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.managing_editor(Some("chris@jupiterbroadcasting.com (Chris Fisher)".to_owned()));
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
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.web_master(Some("chris@jupiterbroadcasting.com (Chris Fisher)".to_owned()));
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
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new().finalize();
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
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.generator(Some("Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_owned()));
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
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new().finalize();
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
        self.ttl = ttl;
        self
    }


    /// Set the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::image::ImageBuilder;
    ///
    /// let image = ImageBuilder::new().finalize();
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
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::text_input::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new().finalize();
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
        self.skip_hours = skip_hours;
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
        self.skip_days = skip_days;
        self
    }


    /// Set the optional items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ChannelBuilder;
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new().title(Some("Making Music with Linux | LAS 408".to_owned())).finalize();
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
    /// let channel = ChannelBuilder::new()
    ///         .title("The Linux Action Show! OGG")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .description("Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!")
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
        Channel {
            title:           self.title.clone(),
            link:            self.link.clone(),
            description:     self.description.clone(),
            language:        self.language.clone(),
            copyright:       self.copyright.clone(),
            managing_editor: self.managing_editor.clone(),
            web_master:      self.web_master.clone(),
            pub_date:        self.pub_date,
            last_build_date: self.last_build_date,
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
