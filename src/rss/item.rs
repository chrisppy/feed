// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
use rss::category::Category;
use rss::enclosure::Enclosure;
use rss::guid::Guid;
use rss::source::Source;
use util;

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
    pub_date:    Option<DateTime<Local>>,
    source:      Option<Source>,
}


impl Item {
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }


    pub fn link(&self) -> Option<String> {
        self.link.clone()
    }


    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }


    pub fn author(&self) -> Option<String> {
        self.author.clone()
    }


    pub fn categories(&self) -> Option<Vec<Category>> {
        self.categories.clone()
    }


    pub fn comments(&self) -> Option<String> {
        self.comments.clone()
    }


    pub fn enclosure(&self) -> Option<Enclosure> {
        self.enclosure.clone()
    }


    pub fn guid(&self) -> Option<Guid> {
        self.guid.clone()
    }


    pub fn pub_date(&self) -> Option<DateTime<Local>> {
        self.pub_date.clone()
    }


    pub fn source(&self) -> Option<Source> {
        self.source.clone()
    }
}


pub struct ItemBuilder {
    title:       Option<String>,
    link:        Option<String>,
    description: Option<String>,
    author:      Option<String>,
    categories:  Option<Vec<Category>>,
    comments:    Option<String>,
    enclosure:   Option<Enclosure>,
    guid:        Option<Guid>,
    pub_date:    Option<DateTime<Local>>,
    source:      Option<Source>,
}


impl ItemBuilder {
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


    pub fn title(&mut self, title: Option<String>) -> &mut ItemBuilder {
        self.title = title;
        self
    }


    pub fn link(&mut self, link: Option<String>) -> &mut ItemBuilder {
        self.link = link;
        self
    }


    pub fn description(&mut self, description: Option<String>) -> &mut ItemBuilder {
        self.description = description;
        self
    }


    pub fn author(&mut self, author: Option<String>) -> &mut ItemBuilder {
        self.author = author;
        self
    }


    pub fn categories(&mut self, categories: Option<Vec<Category>>) -> &mut ItemBuilder {
        self.categories = categories;
        self
    }


    pub fn comments(&mut self, comments: Option<String>) -> &mut ItemBuilder {
        self.comments = comments;
        self
    }


    pub fn enclosure(&mut self, enclosure: Option<Enclosure>) -> &mut ItemBuilder {
        self.enclosure = enclosure;
        self
    }


    pub fn guid(&mut self, guid: Option<Guid>) -> &mut ItemBuilder {
        self.guid = guid;
        self
    }


    pub fn pub_date(&mut self, pub_date: Option<String>) -> &mut ItemBuilder {
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


    pub fn source(&mut self, source: Option<Source>) -> &mut ItemBuilder {
        self.source = source;
        self
    }


    pub fn finalize(&self) -> Item {
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
