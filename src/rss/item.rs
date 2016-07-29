// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The fields under item can be retrieved by using the methods under `Item`.

use chrono::*;
use rss::{Category, Enclosure, Guid, Item, Source};
use url::Url;

impl Item {
    /// Get the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let title_string = "Making Music with Linux | LAS 408".to_owned();
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
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(None)
    ///     .description(Some("A Test Description".to_owned()))
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
    /// use feed::rss::ItemBuilder;
    ///
    /// let link_string = "http://www.jupiterbroadcasting.com/".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some(link_string.clone()))
    ///     .finalize();
    /// let link_option = item.link();
    /// assert!(link_option.is_some());
    /// let link = link_option.unwrap();
    /// assert_eq!(link_string.clone(), link.into_string());
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(None)
    ///     .finalize();
    /// assert!(item.link().is_none());
    /// ```
    pub fn link(&self) -> Option<Url> {
        self.link.clone()
    }


    /// Get the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let description_string = "This is a test description".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .description(Some(description_string.clone()))
    ///     .finalize();
    /// let description_option = item.description();
    /// assert!(description_option.is_some());
    /// let description = description_option.unwrap();
    /// assert_eq!(description_string.clone(), description);
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    /// use feed::rss::ItemBuilder;
    ///
    /// let author_string = "Chris Fisher".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .author(Some(author_string.clone()))
    ///     .finalize();
    /// let author_option = item.author();
    /// assert!(author_option.is_some());
    /// let author = author_option.unwrap();
    /// assert_eq!(author_string.clone(), author);
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    /// use feed::rss::{CategoryBuilder, ItemBuilder};
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
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .categories(Some(categories_vec.clone()))
    ///     .finalize();
    /// let categories_option = item.categories();
    /// assert!(categories_option.is_some());
    /// let categories = categories_option.unwrap();
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    /// use feed::rss::ItemBuilder;
    ///
    /// let comments_string = "http://ekzemplo.com/entry/4403/comments/".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .comments(Some(comments_string.clone()))
    ///     .finalize();
    /// let comments_option =  item.comments();
    /// assert!(comments_option.is_some());
    /// let comments = comments_option.unwrap();
    /// assert_eq!(comments_string.clone(), comments.into_string());
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .comments(None)
    ///     .finalize();
    /// assert!(item.comments().is_none());
    /// ```
    pub fn comments(&self) -> Option<Url> {
        self.comments.clone()
    }


    /// Get the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .length(70772893)
    ///     .enclosure_type("audio/ogg")
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .enclosure(Some(enclosure))
    ///     .finalize();
    ///
    /// assert!(item.enclosure().is_some());
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    /// use feed::rss::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::new()
    ///     .guid("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///     .permalink(None)
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .guid(Some(guid))
    ///     .finalize();
    /// assert!(item.guid().is_some())
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    /// use feed::rss::ItemBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .finalize();
    ///
    /// let local = item.pub_date();
    /// assert!(local.is_some());
    ///
    /// let local_result = local.unwrap();
    /// assert_eq!(pub_date.to_owned(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .pub_date(None)
    ///     .finalize();
    /// assert!(item.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<DateTime<FixedOffset>> {
        self.pub_date
    }


    /// Get the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::{ItemBuilder, SourceBuilder};
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .source("Tomalak's Realm")
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .source(Some(source))
    ///     .finalize();
    /// assert!(item.source().is_some())
    /// ```
    ///
    /// ```
    /// use feed::rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .source(None)
    ///     .finalize();
    /// assert!(item.source().is_none());
    /// ```
    pub fn source(&self) -> Option<Source> {
        self.source.clone()
    }
}
