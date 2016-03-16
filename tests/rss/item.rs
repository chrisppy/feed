// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::category::CategoryBuilder;
use feed::rss::enclosure::EnclosureBuilder;
use feed::rss::guid::GuidBuilder;
use feed::rss::item::ItemBuilder;
use feed::rss::source::SourceBuilder;

#[test]
fn title_some() {
    let title_string = "Making Music with Linux | LAS 408".to_string();
    let item = ItemBuilder::new()
        .title(Some(title_string.clone()))
        .finalize();
    let title_option = item.title();
    assert!(title_option.is_some());
    let title = title_option.unwrap();
    assert_eq!(title_string.clone(), title);
}


#[test]
fn title_none() {
    let item = ItemBuilder::new()
        .title(None)
        .description(Some("A Test Description".to_string()))
        .finalize();
    assert!(item.title().is_none());
}


#[test]
fn link_some() {
    let link_string = "http://www.jupiterbroadcasting.com".to_string();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .link(Some(link_string.clone()))
        .finalize();
    let link_option = item.link();
    assert!(link_option.is_some());
    let link = link_option.unwrap();
    assert_eq!(link_string.clone(), link);
}


#[test]
fn link_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .link(None)
        .finalize();
    assert!(item.link().is_none());
}


#[test]
fn description_some() {
    let description_string = "This is a test description".to_string();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .description(Some(description_string.clone()))
        .finalize();
    let description_option = item.description();
    assert!(description_option.is_some());
    let description = description_option.unwrap();
    assert_eq!(description_string.clone(), description);
}


#[test]
fn description_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .description(None)
        .finalize();
    assert!(item.description().is_none());
}


#[test]
fn author_some() {
    let author_string = "Chris Fisher".to_string();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .author(Some(author_string.clone()))
        .finalize();
    let author_option = item.author();
    assert!(author_option.is_some());
    let author = author_option.unwrap();
    assert_eq!(author_string.clone(), author);
}


#[test]
fn author_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .author(None)
        .finalize();
    assert!(item.author().is_none());
}


#[test]
fn categories_some() {
    let category_1 = CategoryBuilder::new()
        .domain(None)
        .category("Media")
        .finalize();
    let category_2 = CategoryBuilder::new()
        .domain(Some("http://jupiterbroadcasting.com".to_string()))
        .category("Podcast")
        .finalize();
    let categories_vec = vec![category_1, category_2];
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .categories(Some(categories_vec.clone()))
        .finalize();
    let categories_option = item.categories();
    assert!(categories_option.is_some());
    let categories = categories_option.unwrap();
    assert_eq!(categories_vec.clone().len(), categories.len());
}


#[test]
fn categories_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .categories(None)
        .finalize();
    assert!(item.categories().is_none());
}


#[test]
fn comments_some() {
    let comments_string = "This is a test comment.".to_string();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .comments(Some(comments_string.clone()))
        .finalize();
    let comments_option =  item.comments();
    assert!(comments_option.is_some());
    let comments = comments_option.unwrap();
    assert_eq!(comments_string.clone(), comments);

}


#[test]
fn comments_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .comments(None)
        .finalize();
    assert!(item.comments().is_none());
}


#[test]
fn enclosure_some() {
    let enclosure = EnclosureBuilder::new()
        .url("http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg")
        .length(70772893)
        .enclosure_type("audio/ogg")
        .finalize();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .enclosure(Some(enclosure))
        .finalize();
    assert!(item.enclosure().is_some());
}


#[test]
fn enclosure_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .enclosure(None)
        .finalize();
    assert!(item.enclosure().is_none());
}


#[test]
fn guid_some() {
    let guid = GuidBuilder::new()
        .guid("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
        .is_permalink(None)
        .finalize();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .guid(Some(guid))
        .finalize();
    assert!(item.guid().is_some())
}


#[test]
fn guid_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .guid(None)
        .finalize();
    assert!(item.guid().is_none());
}


#[test]
fn pub_date_some() {
    let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    let item = ItemBuilder::new()
        .pub_date(Some(pub_date.to_string()))
        .finalize();
    let local = item.pub_date();
    assert!(local.is_some());
    let local_result = local.unwrap();
    assert_eq!(pub_date.to_string(), local_result.format("%a, %e %b %Y %T %z").to_string());
}


#[test]
fn pub_date_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .pub_date(None)
        .finalize();
    assert!(item.pub_date().is_none());
}


#[test]
fn source_some() {
    let source = SourceBuilder::new()
        .url("http://www.tomalak.org/links2.xml")
        .source("Tomalak's Realm")
        .finalize();
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .source(Some(source))
        .finalize();
    assert!(item.source().is_some())
}


#[test]
fn source_none() {
    let item = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .source(None)
        .finalize();
    assert!(item.source().is_none());
}
