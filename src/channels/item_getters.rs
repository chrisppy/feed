// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under item can be retrieved by using the methods under `Item`.

use channels::ItemGetters;
use rss::{Category, Enclosure, Guid, Item, Source};
use rss::extension::itunes::ITunesItemExtension;


impl ItemGetters for Item
{
    /// Get the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let title_string = "Making Music with Linux | LAS 408".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some(title_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    /// let title_option = item.title();
    /// assert!(title_option.is_some());
    /// let title = title_option.unwrap();
    /// assert_eq!(title_string.clone(), title);
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(None)
    ///     .description(Some("A Test Description".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.title().is_none());
    /// ```
    fn title(&self) -> Option<String>
    {
        self.title.clone()
    }


    /// Get the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let link_string = "http://www.jupiterbroadcasting.com/".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some(link_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    /// let link_option = item.link();
    /// assert!(link_option.is_some());
    ///
    /// let link = link_option.unwrap();
    /// assert_eq!(link_string.clone(), link.into_string());
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.link().is_none());
    /// ```
    fn link(&self) -> Option<String>
    {
        self.link.clone()
    }


    /// Get the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let description_string = "This is a test description".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .description(Some(description_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    /// let description_option = item.description();
    /// assert!(description_option.is_some());
    /// let description = description_option.unwrap();
    /// assert_eq!(description_string.clone(), description);
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .description(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.description().is_none());
    /// ```
    fn description(&self) -> Option<String>
    {
        self.description.clone()
    }


    /// Get the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let author_string = "Chris Fisher".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .author(Some(author_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    /// let author_option = item.author();
    /// assert!(author_option.is_some());
    /// let author = author_option.unwrap();
    /// assert_eq!(author_string.clone(), author);
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.author().is_none());
    /// ```
    fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    /// Get the categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{CategoryBuilder, ItemBuilder};
    ///
    /// let category_1 = CategoryBuilder::new()
    ///     .domain(None)
    ///     .name("Media")
    ///     .finalize()
    ///     .unwrap();
    /// let category_2 = CategoryBuilder::new()
    ///     .domain(Some("http://jupiterbroadcasting.com".to_owned()))
    ///     .name("Podcast")
    ///     .finalize()
    ///     .unwrap();
    /// let categories_vec = vec![category_1, category_2];
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .categories(Some(categories_vec.clone()))
    ///     .finalize()
    ///     .unwrap();
    /// let categories_option = item.categories();
    /// assert!(categories_option.is_some());
    /// let categories = categories_option.unwrap();
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .categories(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.categories().is_none());
    /// ```
    fn categories(&self) -> Vec<Category>
    {
        self.categories.clone()
    }


    /// Get the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let comments_string = "http://example.com/comments".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .comments(Some(comments_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let comments_option =  item.comments();
    /// assert!(comments_option.is_some());
    ///
    /// let comments = comments_option.unwrap();
    /// assert_eq!(comments_string.clone(), comments.into_string());
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .comments(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.comments().is_none());
    /// ```
    fn comments(&self) -> Option<String>
    {
        self.comments.clone()
    }


    /// Get the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .length(70772893)
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .enclosure(Some(enclosure))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.enclosure().is_some());
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .enclosure(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.enclosure().is_none());
    /// ```
    fn enclosure(&self) -> Option<Enclosure>
    {
        self.enclosure.clone()
    }


    /// Get the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::new()
    ///     .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///     .permalink(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .guid(Some(guid))
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.guid().is_some())
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .guid(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.guid().is_none());
    /// ```
    fn guid(&self) -> Option<Guid>
    {
        self.guid.clone()
    }


    /// Get the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = item.pub_date();
    /// assert!(local.is_some());
    ///
    /// let local_result = local.unwrap();
    /// assert_eq!(pub_date.to_owned(), local_result.to_rfc2822());
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .pub_date(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.pub_date().is_none());
    /// ```
    fn pub_date(&self) -> Option<String>
    {
        self.pub_date.clone()
    }


    /// Get the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::{ItemBuilder, SourceBuilder};
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .title(Some("Tomalak's Realm".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .source(Some(source))
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.source().is_some())
    /// ```
    ///
    /// ```
    /// use feed::channels::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.source().is_none());
    /// ```
    fn source(&self) -> Option<Source>
    {
        self.source.clone()
    }


    /// Get the optional `ITunesItemExtension` under `Item`.
    fn itunes_ext(&self) -> Option<ITunesItemExtension>
    {
        self.itunes_ext.clone()
    }
}
