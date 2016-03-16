// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::ChannelBuilder;
use feed::rss::category::CategoryBuilder;

#[test]
fn channel_title() {
    let title = "The Linux Action Show! OGG";
    let channel = ChannelBuilder::new().title(title).finalize();
    assert_eq!(title.to_string(), channel.title());
}


#[test]
fn channel_link() {
    let link = "http://www.jupiterbroadcasting.com";
    let channel = ChannelBuilder::new().link(link).finalize();
    assert_eq!(link.to_string(), channel.link());
}


#[test]
fn channel_description() {
    let description = "Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!";
    let channel = ChannelBuilder::new().description(description).finalize();
    assert_eq!(description.to_string(), channel.description());
}


#[test]
fn channel_language_some() {
    let language_string = "en".to_string();
    let channel = ChannelBuilder::new().language(Some(language_string.clone())).finalize();
    let language_option = channel.language();
    assert!(language_option.is_some());
    let language = language_option.unwrap();
    assert_eq!(language_string.clone(), language);
}


#[test]
fn channel_language_none() {
    let channel = ChannelBuilder::new().language(None).finalize();
    assert!(channel.language().is_none());
}


#[test]
fn channel_copyright_some() {
    let copyright_string = "Copyright 2002, Spartanburg Herald-Journal".to_string();
    let channel = ChannelBuilder::new().copyright(Some(copyright_string.clone())).finalize();
    let copyright_option = channel.copyright();
    assert!(copyright_option.is_some());
    let copyright = copyright_option.unwrap();
    assert_eq!(copyright_string.clone(), copyright);
}


#[test]
fn channel_copyright_none() {
    let channel = ChannelBuilder::new().copyright(None).finalize();
    assert!(channel.copyright().is_none());
}


#[test]
fn channel_managing_editor_some() {
    let managing_editor_string = "chris@jupiterbroadcasting.com (Chris Fisher)".to_string();
    let channel = ChannelBuilder::new().managing_editor(Some(managing_editor_string.clone())).finalize();
    let managing_editor_option = channel.managing_editor();
    assert!(managing_editor_option.is_some());
    let managing_editor = managing_editor_option.unwrap();
    assert_eq!(managing_editor_string.clone(), managing_editor);
}


#[test]
fn channel_managing_editor_none() {
    let channel = ChannelBuilder::new().managing_editor(None).finalize();
    assert!(channel.managing_editor().is_none());
}


#[test]
fn channel_web_master_some() {
    let web_master_string = "chris@jupiterbroadcasting.com (Chris Fisher)".to_string();
    let channel = ChannelBuilder::new().web_master(Some(web_master_string.clone())).finalize();
    let web_master_option = channel.web_master();
    assert!(web_master_option.is_some());
    let web_master = web_master_option.unwrap();
    assert_eq!(web_master_string.clone(), web_master);
}


#[test]
fn channel_web_master_none() {
    let channel = ChannelBuilder::new().web_master(None).finalize();
    assert!(channel.web_master().is_none());
}


#[test]
fn channel_pub_date_some() {
    let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    let channel = ChannelBuilder::new().pub_date(Some(pub_date.to_string())).finalize();
    let local = channel.pub_date();
    assert!(local.is_some());
    let local_result = local.unwrap();
    assert_eq!(pub_date.to_string(), local_result.format("%a, %e %b %Y %T %z").to_string());
}


#[test]
fn channel_pub_date_none() {
    let channel = ChannelBuilder::new().pub_date(None).finalize();
    assert!(channel.pub_date().is_none());
}


#[test]
fn channel_last_build_date_some() {
    let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    let channel = ChannelBuilder::new().last_build_date(Some(last_build_date.to_string())).finalize();
    let local = channel.last_build_date();
    assert!(local.is_some());
    let local_result = local.unwrap();
    assert_eq!(last_build_date.to_string(), local_result.format("%a, %e %b %Y %T %z").to_string());
}


#[test]
fn channel_last_build_date_none() {
    let channel = ChannelBuilder::new().last_build_date(None).finalize();
    assert!(channel.last_build_date().is_none());
}


#[test]
fn channel_categories_some() {
    let category_1 = CategoryBuilder::new().domain(None).category("Media").finalize();
    let category_2 = CategoryBuilder::new().domain(Some("http://jupiterbroadcasting.com".to_string())).category("Podcast").finalize();
    let categories_vec = vec![category_1, category_2];
    let channel = ChannelBuilder::new().categories(Some(categories_vec.clone())).finalize();
    let categories_option = channel.categories();
    assert!(categories_option.is_some());
    let categories = categories_option.unwrap();
    assert_eq!(categories_vec.clone().len(), categories.len());
}


#[test]
fn channel_categories_none() {
    let channel = ChannelBuilder::new().categories(None).finalize();
    assert!(channel.categories().is_none());
}


#[test]
fn channel_generator_some() {
    let generator_string = "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_string();
    let channel = ChannelBuilder::new().generator(Some(generator_string.clone())).finalize();
    let generator_option = channel.generator();
    assert!(generator_option.is_some());
    let generator = generator_option.unwrap();
    assert_eq!(generator_string.clone(), generator);
}


#[test]
fn channel_generator_none() {
    let channel = ChannelBuilder::new().generator(None).finalize();
    assert!(channel.generator().is_none());
}


#[test]
fn channel_docs_some() {
    let docs_string = "http://blogs.law.harvard.edu/tech/rss".to_string();
    let channel = ChannelBuilder::new().docs(Some(docs_string.clone())).finalize();
    let docs_option = channel.docs();
    assert!(docs_option.is_some());
    let docs = docs_option.unwrap();
    assert_eq!(docs_string.clone(), docs);
}


#[test]
fn channel_docs_none() {
    let channel = ChannelBuilder::new().docs(None).finalize();
    assert!(channel.docs().is_none());
}
