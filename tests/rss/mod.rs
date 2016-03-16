// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod category;
mod cloud;
mod enclosure;
mod guid;
mod image;
mod item;
mod source;
mod text_input;

use feed::rss::ChannelBuilder;
use feed::rss::category::CategoryBuilder;
use feed::rss::cloud::CloudBuilder;
use feed::rss::image::ImageBuilder;
use feed::rss::item::ItemBuilder;
use feed::rss::text_input::TextInputBuilder;

#[test]
fn title() {
    let title = "The Linux Action Show! OGG";
    let channel = ChannelBuilder::new()
        .title(title)
        .finalize();
    assert_eq!(title.to_string(), channel.title());
}


#[test]
fn link() {
    let link = "http://www.jupiterbroadcasting.com";
    let channel = ChannelBuilder::new()
        .link(link)
        .finalize();
    assert_eq!(link.to_string(), channel.link());
}


#[test]
fn description() {
    let description = "Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!";
    let channel = ChannelBuilder::new()
        .description(description)
        .finalize();
    assert_eq!(description.to_string(), channel.description());
}


#[test]
fn language_some() {
    let language_string = "en".to_string();
    let channel = ChannelBuilder::new()
        .language(Some(language_string.clone()))
        .finalize();
    let language_option = channel.language();
    assert!(language_option.is_some());
    let language = language_option.unwrap();
    assert_eq!(language_string.clone(), language);
}


#[test]
fn language_none() {
    let channel = ChannelBuilder::new()
        .language(None)
        .finalize();
    assert!(channel.language().is_none());
}


#[test]
fn copyright_some() {
    let copyright_string = "Copyright 2002, Spartanburg Herald-Journal".to_string();
    let channel = ChannelBuilder::new()
        .copyright(Some(copyright_string.clone()))
        .finalize();
    let copyright_option = channel.copyright();
    assert!(copyright_option.is_some());
    let copyright = copyright_option.unwrap();
    assert_eq!(copyright_string.clone(), copyright);
}


#[test]
fn copyright_none() {
    let channel = ChannelBuilder::new()
        .copyright(None)
        .finalize();
    assert!(channel.copyright().is_none());
}


#[test]
fn managing_editor_some() {
    let managing_editor_string = "chris@jupiterbroadcasting.com (Chris Fisher)".to_string();
    let channel = ChannelBuilder::new()
        .managing_editor(Some(managing_editor_string.clone()))
        .finalize();
    let managing_editor_option = channel.managing_editor();
    assert!(managing_editor_option.is_some());
    let managing_editor = managing_editor_option.unwrap();
    assert_eq!(managing_editor_string.clone(), managing_editor);
}


#[test]
fn managing_editor_none() {
    let channel = ChannelBuilder::new()
        .managing_editor(None)
        .finalize();
    assert!(channel.managing_editor().is_none());
}


#[test]
fn web_master_some() {
    let web_master_string = "chris@jupiterbroadcasting.com (Chris Fisher)".to_string();
    let channel = ChannelBuilder::new()
        .web_master(Some(web_master_string.clone()))
        .finalize();
    let web_master_option = channel.web_master();
    assert!(web_master_option.is_some());
    let web_master = web_master_option.unwrap();
    assert_eq!(web_master_string.clone(), web_master);
}


#[test]
fn web_master_none() {
    let channel = ChannelBuilder::new()
        .web_master(None)
        .finalize();
    assert!(channel.web_master().is_none());
}


#[test]
fn pub_date_some() {
    let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    let channel = ChannelBuilder::new()
        .pub_date(Some(pub_date.to_string()))
        .finalize();
    let local = channel.pub_date();
    assert!(local.is_some());
    let local_result = local.unwrap();
    assert_eq!(pub_date.to_string(), local_result.format("%a, %e %b %Y %T %z").to_string());
}


#[test]
fn pub_date_none() {
    let channel = ChannelBuilder::new()
        .pub_date(None)
        .finalize();
    assert!(channel.pub_date().is_none());
}


#[test]
fn last_build_date_some() {
    let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    let channel = ChannelBuilder::new()
        .last_build_date(Some(last_build_date.to_string()))
        .finalize();
    let local = channel.last_build_date();
    assert!(local.is_some());
    let local_result = local.unwrap();
    assert_eq!(last_build_date.to_string(), local_result.format("%a, %e %b %Y %T %z").to_string());
}


#[test]
fn last_build_date_none() {
    let channel = ChannelBuilder::new()
        .last_build_date(None)
        .finalize();
    assert!(channel.last_build_date().is_none());
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
    let channel = ChannelBuilder::new()
        .categories(Some(categories_vec.clone()))
        .finalize();
    let categories_option = channel.categories();
    assert!(categories_option.is_some());
    let categories = categories_option.unwrap();
    assert_eq!(categories_vec.clone().len(), categories.len());
}


#[test]
fn categories_none() {
    let channel = ChannelBuilder::new()
        .categories(None)
        .finalize();
    assert!(channel.categories().is_none());
}


#[test]
fn generator_some() {
    let generator_string = "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) http://reinventedsoftware.com/feeder/".to_string();
    let channel = ChannelBuilder::new()
        .generator(Some(generator_string.clone()))
        .finalize();
    let generator_option = channel.generator();
    assert!(generator_option.is_some());
    let generator = generator_option.unwrap();
    assert_eq!(generator_string.clone(), generator);
}


#[test]
fn generator_none() {
    let channel = ChannelBuilder::new()
        .generator(None)
        .finalize();
    assert!(channel.generator().is_none());
}


#[test]
fn docs_some() {
    let docs_string = "http://blogs.law.harvard.edu/tech/rss".to_string();
    let channel = ChannelBuilder::new()
        .docs(Some(docs_string.clone()))
        .finalize();
    let docs_option = channel.docs();
    assert!(docs_option.is_some());
    let docs = docs_option.unwrap();
    assert_eq!(docs_string.clone(), docs);
}


#[test]
fn docs_none() {
    let channel = ChannelBuilder::new()
        .docs(None)
        .finalize();
    assert!(channel.docs().is_none());
}


#[test]
fn cloud_some() {
    let cloud = CloudBuilder::new()
        .domain("rpc.sys.com")
        .port(80)
        .path("/RPC2")
        .register_procedure("pingMe")
        .protocol("soap")
        .finalize();
    let channel = ChannelBuilder::new()
        .cloud(Some(cloud))
        .finalize();
    assert!(channel.cloud().is_some());
}


#[test]
fn cloud_none() {
    let channel = ChannelBuilder::new()
        .cloud(None)
        .finalize();
    assert!(channel.cloud().is_none());
}


#[test]
fn ttl_some() {
    let ttl_num = 60;
    let channel = ChannelBuilder::new()
        .ttl(Some(ttl_num))
        .finalize();
    let ttl_option = channel.ttl();
    assert!(ttl_option.is_some());
    let ttl = ttl_option.unwrap();
    assert_eq!(ttl_num, ttl);
}


#[test]
fn ttl_none() {
    let channel = ChannelBuilder::new()
        .ttl(None)
        .finalize();
    assert!(channel.ttl().is_none());
}


#[test]
fn image_some() {
    let image = ImageBuilder::new()
        .link("http://www.jupiterbroadcasting.com")
        .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
        .title("LAS 300 Logo")
        .height(None)
        .width(None)
        .description(None)
        .finalize();
    let channel = ChannelBuilder::new()
        .image(Some(image))
        .finalize();
    assert!(channel.image().is_some());
}


#[test]
fn image_none() {
    let channel = ChannelBuilder::new()
        .image(None)
        .finalize();
    assert!(channel.image().is_none());
}


#[test]
fn rating_some() {
    let rating_string = "PG-13:".to_string();
    let channel = ChannelBuilder::new()
        .rating(Some(rating_string.clone()))
        .finalize();
    let rating_option = channel.rating();
    assert!(rating_option.is_some());
    let rating = rating_option.unwrap();
    assert_eq!(rating_string, rating);
}


#[test]
fn rating_none() {
    let channel = ChannelBuilder::new()
        .rating(None)
        .finalize();
    assert!(channel.rating().is_none());
}


#[test]
fn text_input_some() {
    let text_input = TextInputBuilder::new()
        .title("Enter Comment")
        .description("Provided Feedback")
        .name("Comment")
        .link("www.example.com/feedback")
        .finalize();
    let channel = ChannelBuilder::new()
        .text_input(Some(text_input))
        .finalize();
    assert!(channel.text_input().is_some());
}


#[test]
fn text_input_none() {
    let channel = ChannelBuilder::new()
        .text_input(None)
        .finalize();
    assert!(channel.text_input().is_none());
}


#[test]
fn skip_hours_some() {
    let skip_hours_vec: Vec<i64> = vec![6,7,8,14,22];
    let channel = ChannelBuilder::new()
        .skip_hours(Some(skip_hours_vec.clone()))
        .finalize();
    let skip_hours_option = channel.skip_hours();
    assert!(skip_hours_option.is_some());
    let skip_hours = skip_hours_option.unwrap();
    let len = skip_hours_vec.clone().len();
    assert_eq!(len, skip_hours.len());
    for x in 0..len {
        assert_eq!(skip_hours_vec[x], skip_hours[x]);
    }
}


#[test]
fn skip_hours_none() {
    let channel = ChannelBuilder::new()
        .skip_hours(None)
        .finalize();
    assert!(channel.skip_hours().is_none());
}


#[test]
fn skip_days_some() {
    let skip_days_vec: Vec<String> = vec!["Monday".to_string(),"Wednesday".to_string(),"Thursday".to_string(),"Sunday".to_string()];
    let channel = ChannelBuilder::new()
        .skip_days(Some(skip_days_vec.clone()))
        .finalize();
    let skip_days_option = channel.skip_days();
    assert!(skip_days_option.is_some());
    let skip_days = skip_days_option.unwrap();
    let len = skip_days_vec.clone().len();
    assert_eq!(len, skip_days.len());
    for x in 0..len {
        assert_eq!(skip_days_vec[x], skip_days[x]);
    }
}


#[test]
fn skip_days_none() {
    let channel = ChannelBuilder::new()
        .skip_days(None)
        .finalize();
    assert!(channel.skip_days().is_none());
}


#[test]
fn items_some() {
    let item_1 = ItemBuilder::new()
        .title(Some("Making Music with Linux | LAS 408".to_string()))
        .link(Some("http://www.jupiterbroadcasting.com/97561/making-music-with-linux-las-408/".to_string()))
        .description(None)
        .author(None)
        .categories(None)
        .enclosure(None)
        .guid(None)
        .pub_date(None)
        .source(None)
        .finalize();
    let item_2 = ItemBuilder::new()
        .title(None)
        .link(None)
        .description(Some("<![CDATA[<p>In special Rasberry Pi 3 edition of the show we look at the new hardware, review & chat with Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for development of their open artificial intelligence platform & get the latest news.</p>

<p>Plus replacing Spotify on Linux, the new Microsoft lock-in, our hosts face a moral quandary & more!</p>]]>".to_string()))
        .author(None)
        .categories(None)
        .enclosure(None)
        .guid(None)
        .pub_date(None)
        .source(None)
        .finalize();
    let items_vec = vec![item_1, item_2];
    let channel = ChannelBuilder::new()
        .items(Some(items_vec.clone()))
        .finalize();
    let items_option = channel.items();
    assert!(items_option.is_some());
    let items = items_option.unwrap();
    assert_eq!(items_vec.clone().len(), items.len());
}


#[test]
fn items_none() {
    let channel = ChannelBuilder::new()
        .items(None)
        .finalize();
    assert!(channel.items().is_none());
}
