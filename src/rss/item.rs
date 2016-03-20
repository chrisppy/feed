// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
use rss::category::Category;
use rss::enclosure::Enclosure;
use rss::guid::Guid;
use rss::source::Source;
use util;

/// This `Item` struct contains all the items that exist for the guid field under 'Channel'.
#[derive(Clone)]
pub struct Item {
    title:       Option<String>,
    link:        Option<String>,
    description: Option<String>,
    author:      Option<String>,
    categories:  Option<Vec<Category>>,
    comments:    Option<String>,
    enclosure:   Option<Enclosure>,
    guid:        Option<Guid>,
    pub_date:    Option<DateTime<FixedOffset>>,
    source:      Option<Source>,
}


impl Item {
    /// Get the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let title_string = "Making Music with Linux | LAS 408".to_string();
    /// let item = ItemBuilder::new()
    ///     .title(Some(title_string.clone()))
    ///     .finalize();
    /// let title_option = item.title();
    /// assert!(title_option.is_some());
    /// let title = title_option.unwrap();
    /// assert_eq!(title_string.clone(), title);
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(None)
    ///     .description(Some("A Test Description".to_string()))
    ///     .finalize();
    /// assert!(item.title().is_none());
    /// ```
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }


    /// Get the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let link_string = "http://www.jupiterbroadcasting.com".to_string();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .link(Some(link_string.clone()))
    ///     .finalize();
    /// let link_option = item.link();
    /// assert!(link_option.is_some());
    /// let link = link_option.unwrap();
    /// assert_eq!(link_string.clone(), link);
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .link(None)
    ///     .finalize();
    /// assert!(item.link().is_none());
    /// ```
    pub fn link(&self) -> Option<String> {
        self.link.clone()
    }


    /// Get the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let description_string = "This is a test description".to_string();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .description(Some(description_string.clone()))
    ///     .finalize();
    /// let description_option = item.description();
    /// assert!(description_option.is_some());
    /// let description = description_option.unwrap();
    /// assert_eq!(description_string.clone(), description);
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .description(None)
    ///     .finalize();
    /// assert!(item.description().is_none());
    /// ```
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }


    /// Get the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let author_string = "Chris Fisher".to_string();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .author(Some(author_string.clone()))
    ///     .finalize();
    /// let author_option = item.author();
    /// assert!(author_option.is_some());
    /// let author = author_option.unwrap();
    /// assert_eq!(author_string.clone(), author);
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .author(None)
    ///     .finalize();
    /// assert!(item.author().is_none());
    /// ```
    pub fn author(&self) -> Option<String> {
        self.author.clone()
    }


    /// Get the optional categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category_1 = CategoryBuilder::new()
    ///     .domain(None)
    ///     .category("Media")
    ///     .finalize();
    /// let category_2 = CategoryBuilder::new()
    ///     .domain(Some("http://jupiterbroadcasting.com".to_string()))
    ///     .category("Podcast")
    ///     .finalize();
    /// let categories_vec = vec![category_1, category_2];
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .categories(Some(categories_vec.clone()))
    ///     .finalize();
    /// let categories_option = item.categories();
    /// assert!(categories_option.is_some());
    /// let categories = categories_option.unwrap();
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .categories(None)
    ///     .finalize();
    /// assert!(item.categories().is_none());
    /// ```
    pub fn categories(&self) -> Option<Vec<Category>> {
        self.categories.clone()
    }


    /// Get the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let comments_string = "This is a test comment.".to_string();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .comments(Some(comments_string.clone()))
    ///     .finalize();
    /// let comments_option =  item.comments();
    /// assert!(comments_option.is_some());
    /// let comments = comments_option.unwrap();
    /// assert_eq!(comments_string.clone(), comments);
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .comments(None)
    ///     .finalize();
    /// assert!(item.comments().is_none());
    /// ```
    pub fn comments(&self) -> Option<String> {
        self.comments.clone()
    }


    /// Get the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url("http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg")
    ///     .length(70772893)
    ///     .enclosure_type("audio/ogg")
    ///     .finalize();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .enclosure(Some(enclosure))
    ///     .finalize();
    /// assert!(item.enclosure().is_some());
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .enclosure(None)
    ///     .finalize();
    /// assert!(item.enclosure().is_none());
    /// ```
    pub fn enclosure(&self) -> Option<Enclosure> {
        self.enclosure.clone()
    }


    /// Get the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///     .guid("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///     .is_permalink(None)
    ///     .finalize();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .guid(Some(guid))
    ///     .finalize();
    /// assert!(item.guid().is_some())
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .guid(None)
    ///     .finalize();
    /// assert!(item.guid().is_none());
    /// ```
    pub fn guid(&self) -> Option<Guid> {
        self.guid.clone()
    }


    /// Get the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .pub_date(Some(pub_date.to_string()))
    ///     .finalize();
    /// let local = item.pub_date();
    /// assert!(local.is_some());
    /// let local_result = local.unwrap();
    /// assert_eq!(pub_date.to_string(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .pub_date(None)
    ///     .finalize();
    /// assert!(item.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<DateTime<FixedOffset>> {
        self.pub_date.clone()
    }


    /// Get the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .source("Tomalak's Realm")
    ///     .finalize();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .source(Some(source))
    ///     .finalize();
    /// assert!(item.source().is_some())
    /// ```
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .source(None)
    ///     .finalize();
    /// assert!(item.source().is_none());
    /// ```
    pub fn source(&self) -> Option<Source> {
        self.source.clone()
    }
}


/// This `ItemBuilder` struct creates the `Item`.
pub struct ItemBuilder {
    title:       Option<String>,
    link:        Option<String>,
    description: Option<String>,
    author:      Option<String>,
    categories:  Option<Vec<Category>>,
    comments:    Option<String>,
    enclosure:   Option<Enclosure>,
    guid:        Option<Guid>,
    pub_date:    Option<DateTime<FixedOffset>>,
    source:      Option<Source>,
}


impl ItemBuilder {
    /// Construct a new `ItemBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new();
    /// ```
    pub fn new() -> ItemBuilder {
        ItemBuilder {
            title:       None,
            link:        None,
            description: None,
            author:      None,
            categories:  None,
            comments:    None,
            enclosure:   None,
            guid:        None,
            pub_date:    None,
            source:      None,
        }
    }


    /// Set the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.title(Some("Making Music with Linux | LAS 408".to_string()));
    /// ```
    pub fn title(&mut self, title: Option<String>) -> &mut ItemBuilder {
        self.title = title;
        self
    }


    /// Set the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.link(Some("http://www.jupiterbroadcasting.com".to_string()));
    /// ```
    pub fn link(&mut self, link: Option<String>) -> &mut ItemBuilder {
        self.link = link;
        self
    }


    /// Set the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.description(Some("This is a test description".to_string()));
    /// ```
    pub fn description(&mut self, description: Option<String>) -> &mut ItemBuilder {
        self.description = description;
        self
    }


    /// Set the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.author(Some("Chris Fisher".to_string()));
    /// ```
    pub fn author(&mut self, author: Option<String>) -> &mut ItemBuilder {
        self.author = author;
        self
    }


    /// Set the optional categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::category::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new().finalize();
    /// let categories = vec![category];
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.categories(Some(categories));
    /// ```
    pub fn categories(&mut self, categories: Option<Vec<Category>>) -> &mut ItemBuilder {
        self.categories = categories;
        self
    }


    /// Set the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.comments(Some("Test Comment".to_string()));
    /// ```
    pub fn comments(&mut self, comments: Option<String>) -> &mut ItemBuilder {
        self.comments = comments;
        self
    }


    /// Set the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::enclosure::EnclosureBuilder;
    ///
    /// let enclosure = EnclosureBuilder::new().finalize();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.enclosure(Some(enclosure));
    /// ```
    pub fn enclosure(&mut self, enclosure: Option<Enclosure>) -> &mut ItemBuilder {
        self.enclosure = enclosure;
        self
    }


    /// Set the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::guid::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new().finalize();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.guid(Some(guid));
    /// ```
    pub fn guid(&mut self, guid: Option<Guid>) -> &mut ItemBuilder {
        self.guid = guid;
        self
    }


    /// Set the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let mut item_builder = feed::rss::item::ItemBuilder::new();
    /// item_builder.pub_date(Some("Sun, 13 Mar 2016 20:02:02 -0700".to_string()));
    /// ```
    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ItemBuilder {
        self.pub_date = util::option_string_to_option_date(pub_date);
        self
    }


    /// Set the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    /// use feed::rss::source::SourceBuilder;
    ///
    /// let source = SourceBuilder::new().finalize();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.source(Some(source));
    /// ```
    pub fn source(&mut self, source: Option<Source>) -> &mut ItemBuilder {
        self.source = source;
        self
    }


    /// Construct the `Item` from the `ItemBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::item::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///         .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///         .link(Some("http://www.jupiterbroadcasting.com".to_string()))
    ///         .description(None)
    ///         .author(None)
    ///         .categories(None)
    ///         .comments(None)
    ///         .enclosure(None)
    ///         .guid(None)
    ///         .pub_date(None)
    ///         .source(None)
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Item {
        if self.title.is_none() && self.description.is_none() {
            panic!("Either Title or Description must have a value!");
        }
        Item {
            title:       self.title.clone(),
            link:        self.link.clone(),
            description: self.description.clone(),
            author:      self.author.clone(),
            categories:  self.categories.clone(),
            comments:    self.comments.clone(),
            enclosure:   self.enclosure.clone(),
            guid:        self.guid.clone(),
            pub_date:    self.pub_date.clone(),
            source:      self.source.clone(),
        }
    }
}
