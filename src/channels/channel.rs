// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under channels can be retrieved by using the methods under `Channel`.


use chrono::*;
use channels::{Category, Channel, Cloud, Image, Item, TextInput};
use enums::Day;
use url::Url;


impl Channel {
    /// Get the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let title = "The Linux Action Show! OGG";
    /// let channels = ChannelBuilder::new()
    ///     .title(title)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert_eq!(title.to_owned(), channels.title());
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }


    /// Get the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    /// let channels = ChannelBuilder::new()
    ///     .link(link)
    ///     .finalize();
    /// assert_eq!(link.to_owned(), channels.link().into_string());
    /// ```
    pub fn link(&self) -> Url {
        self.link.clone()
    }


    /// Get the description that exists under `Channel`.
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
    /// let channels = ChannelBuilder::new()
    ///     .description(description.as_ref())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert_eq!(description.to_owned(), channels.description());
    /// ```
    pub fn description(&self) -> String {
        self.description.clone()
    }


    /// Get the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let language_string = "en".to_owned();
    /// let channels = ChannelBuilder::new()
    ///     .language(Some(language_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let language_option = channels.language();
    /// assert!(language_option.is_some());
    /// let language = language_option.unwrap();
    /// assert_eq!(language_string.clone(), language);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.language().is_none());
    /// ```
    pub fn language(&self) -> Option<String> {
        self.language.clone()
    }


    /// Get the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let copyright_string =
    ///     "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    /// let channels = ChannelBuilder::new()
    ///     .copyright(Some(copyright_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let copyright_option = channels.copyright();
    /// assert!(copyright_option.is_some());
    /// let copyright = copyright_option.unwrap();
    /// assert_eq!(copyright_string.clone(), copyright);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.copyright().is_none());
    /// ```
    pub fn copyright(&self) -> Option<String> {
        self.copyright.clone()
    }


    /// Get the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let managing_editor_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(Some(managing_editor_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let managing_editor_option = channels.managing_editor();
    /// assert!(managing_editor_option.is_some());
    /// let managing_editor = managing_editor_option.unwrap();
    /// assert_eq!(managing_editor_string.clone(), managing_editor);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.managing_editor().is_none());
    /// ```
    pub fn managing_editor(&self) -> Option<String> {
        self.managing_editor.clone()
    }

    /// Get the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let web_master_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    /// let channels = ChannelBuilder::new()
    ///     .web_master(Some(web_master_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let web_master_option = channels.web_master();
    /// assert!(web_master_option.is_some());
    /// let web_master = web_master_option.unwrap();
    /// assert_eq!(web_master_string.clone(), web_master);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .web_master(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.web_master().is_none());
    /// ```
    pub fn web_master(&self) -> Option<String> {
        self.web_master.clone()
    }


    /// Get the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let local = channels.pub_date();
    /// assert!(local.is_some());
    /// let local_result = local.unwrap();
    /// assert_eq!(pub_date.to_owned(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<DateTime<FixedOffset>> {
        self.pub_date
    }


    /// Get the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(Some(last_build_date.to_owned()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let local = channels.last_build_date();
    /// assert!(local.is_some());
    /// let local_result = local.unwrap();
    /// assert_eq!(last_build_date.to_owned(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.last_build_date().is_none());
    /// ```
    pub fn last_build_date(&self) -> Option<DateTime<FixedOffset>> {
        self.last_build_date
    }


    /// Get the optional categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, CategoryBuilder};
    ///
    /// let category_1 = CategoryBuilder::new()
    ///     .domain(None)
    ///     .name("Media")
    ///     .finalize();
    /// let category_2 = CategoryBuilder::new()
    ///     .domain(Some("http://jupiterbroadcasting.com".to_owned()))
    ///     .name("Podcast")
    ///     .finalize();
    /// let categories_vec = vec![category_1, category_2];
    /// let channels = ChannelBuilder::new()
    ///     .categories(Some(categories_vec.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let categories_option = channels.categories();
    /// assert!(categories_option.is_some());
    /// let categories = categories_option.unwrap();
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .categories(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.categories().is_none());
    /// ```
    pub fn categories(&self) -> Option<Vec<Category>> {
        self.categories.clone()
    }


    /// Get the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let generator_string = "Feeder 2.5.12(2294); ".to_owned()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(Some(generator_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let generator_option = channels.generator();
    /// assert!(generator_option.is_some());
    /// let generator = generator_option.unwrap();
    /// assert_eq!(generator_string.clone(), generator);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.generator().is_none());
    /// ```
    pub fn generator(&self) -> Option<String> {
        self.generator.clone()
    }


    /// Get the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let docs_string = "http://blogs.law.harvard.edu/tech/rss/".to_owned();
    /// let channels = ChannelBuilder::new()
    ///     .docs(Some(docs_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let docs_option = channels.docs();
    /// assert!(docs_option.is_some());
    /// let docs = docs_option.unwrap();
    /// assert_eq!(docs_string.clone(), docs.into_string());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.docs().is_none());
    /// ```
    pub fn docs(&self) -> Option<Url> {
        self.docs.clone()
    }

    /// Get the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("http://rpc.sys.com/")
    ///     .port(80)
    ///     .path("/RPC2")
    ///     .register_procedure("pingMe")
    ///     .protocol("soap")
    ///     .finalize();
    /// let channels = ChannelBuilder::new()
    ///     .cloud(Some(cloud))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.cloud().is_some());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .cloud(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.cloud().is_none());
    /// ```
    pub fn cloud(&self) -> Option<Cloud> {
        self.cloud.clone()
    }


    /// Get the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let ttl_num = 60;
    /// let channels = ChannelBuilder::new()
    ///     .ttl(Some(ttl_num))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let ttl_option = channels.ttl();
    /// assert!(ttl_option.is_some());
    /// let ttl = ttl_option.unwrap();
    /// assert_eq!(ttl_num, ttl);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .ttl(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.ttl().is_none());
    /// ```
    pub fn ttl(&self) -> Option<i64> {
        self.ttl
    }


    /// Get the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, ImageBuilder};
    ///
    /// let image = ImageBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com")
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .height(None)
    ///     .width(None)
    ///     .description(None)
    ///     .finalize();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .image(Some(image))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.image().is_some());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .image(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.image().is_none());
    /// ```
    pub fn image(&self) -> Option<Image> {
        self.image.clone()
    }


    /// Get the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let rating_string = "PG-13:".to_owned();
    /// let channels = ChannelBuilder::new()
    ///     .rating(Some(rating_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let rating_option = channels.rating();
    /// assert!(rating_option.is_some());
    /// let rating = rating_option.unwrap();
    /// assert_eq!(rating_string, rating);
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .rating(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.rating().is_none());
    /// ```
    pub fn rating(&self) -> Option<String> {
        self.rating.clone()
    }


    /// Get the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .title("Enter Comment")
    ///     .description("Provided Feedback")
    ///     .name("Comment")
    ///     .link("http://www.example.com/feedback")
    ///     .finalize();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .text_input(Some(text_input))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.text_input().is_some());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .text_input(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.text_input().is_none());
    /// ```
    pub fn text_input(&self) -> Option<TextInput> {
        self.text_input.clone()
    }

    /// Get the optional skip hours that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let skip_hours_vec: Vec<i64> = vec![6,7,8,14,22];
    /// let channels = ChannelBuilder::new()
    ///     .skip_hours(Some(skip_hours_vec.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let skip_hours_option = channels.skip_hours();
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
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_hours(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.skip_hours().is_none());
    /// ```
    pub fn skip_hours(&self) -> Option<Vec<i64>> {
        self.skip_hours.clone()
    }


    /// Get the optional skip days that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let skip_days_vec: Vec<String> = vec!["Monday".to_owned(),
    ///     "Wednesday".to_owned(),"Thursday".to_owned(),"Sunday".to_owned()];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_days(Some(skip_days_vec.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    ///
    /// let skip_days_option = channels.skip_days();
    /// assert!(skip_days_option.is_some());
    ///
    /// let skip_days = skip_days_option.unwrap();
    /// let len = skip_days_vec.clone().len();
    /// assert_eq!(len, skip_days.len());
    ///
    /// for x in 0..len {
    ///     let day = skip_days[x].clone();
    ///     assert_eq!(skip_days_vec[x], day.into_string());
    /// }
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_days(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.skip_days().is_none());
    /// ```
    pub fn skip_days(&self) -> Option<Vec<Day>> {
        self.skip_days.clone()
    }


    /// Get the optional items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ChannelBuilder, ItemBuilder};
    ///
    /// let link = "http://www.jupiterbroadcasting.com/97561/".to_owned()
    /// + "making-music-with-linux-las-408/";
    ///
    /// let description = "<![CDATA[<p>In special Rasberry Pi 3 ".to_owned()
    /// + "edition of the show we look at the new hardware, review & chat with "
    /// + "Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for "
    /// + "development of their open artificial intelligence platform & get "
    /// + "the latest news.</p><p>Plus replacing Spotify on Linux, the new "
    /// + "Microsoft lock-in, our hosts face a moral quandary & more!</p>]]>";
    ///
    /// let title = "Making Music with Linux | LAS 408".to_owned();
    ///
    /// let item_1 = ItemBuilder::new()
    ///     .title(Some(title))
    ///     .link(Some(link))
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
    ///     .description(Some(description))
    ///     .author(None)
    ///     .categories(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize();
    /// let items_vec = vec![item_1, item_2];
    /// let channels = ChannelBuilder::new()
    ///     .items(Some(items_vec.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// let items_option = channels.items();
    /// assert!(items_option.is_some());
    /// let items = items_option.unwrap();
    /// assert_eq!(items_vec.clone().len(), items.len());
    /// ```
    ///
    /// ```
    /// use feed::channels::ChannelBuilder;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .items(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    /// assert!(channels.items().is_none());
    /// ```
    pub fn items(&self) -> Option<Vec<Item>> {
        self.items.clone()
    }
}
