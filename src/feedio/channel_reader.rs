// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `ChannelReader`.

use errors;
use feedio::ChannelReader;
use quick_xml::{Event, XmlReader};
use rss::{Channel, ChannelBuilder, Category, CategoryBuilder, CloudBuilder, EnclosureBuilder,
          GuidBuilder, ImageBuilder, Item, ItemBuilder, SourceBuilder, TextInputBuilder};
use std::str::FromStr;
use util;

impl ChannelReader {
    /// Construct a new `ChannelReader` and return the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::ChannelReader;
    ///
    /// let rss = r#"<?xml version="1.0" encoding="UTF-8"?>
    ///             <rss version="2.0">
    ///                  <channel>
    ///                      <title>The Linux Action Show! OGG</title>
    ///                      <link>http://www.jupiterbroadcasting.com</link>
    ///                      <description>Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!</description>
    ///                  </channel>
    ///              </rss>"#;
    ///
    /// ChannelReader::new(rss);
    /// ```
    pub fn new(feed: &str) -> ChannelReader {
        let feed_string = feed.to_owned();
        let mut category_builder = CategoryBuilder::new();
        let mut channel_builder = ChannelBuilder::new();
        let mut guid_builder = GuidBuilder::new();
        let mut image_builder = ImageBuilder::new();
        let mut item_builder = ItemBuilder::new();
        let mut source_builder = SourceBuilder::new();
        let mut text_input_builder = TextInputBuilder::new();

        let mut channel_categories: Vec<Category> = Vec::new();
        let mut channel_skip_days: Vec<String> = Vec::new();
        let mut channel_skip_hours: Vec<i64> = Vec::new();
        let mut items: Vec<Item> = Vec::new();
        let mut item_categories: Vec<Category> = Vec::new();
        let mut element = "channel";
        let mut name = "";

        let mut image_tag_present = false;
        let mut text_input_tag_present = false;

        let reader = XmlReader::from(&*feed_string).trim_text(true);
        for r in reader {
            match r {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"image" => {
                            image_tag_present = true;
                            element = "image";
                        }
                        b"textInput" => {
                            text_input_tag_present = true;
                            element = "textInput";
                        }
                        b"item" => {
                            item_builder = ItemBuilder::new();
                            element = "item";
                        }
                        b"author" => {
                            name = "author";
                        }
                        b"category" => {
                            name = "category";

                            let domain = util::attribute_to_string(e.attributes(), "domain")
                                .to_owned();

                            category_builder = CategoryBuilder::new();
                            category_builder.domain(domain);
                        }
                        b"cloud" => {


                            let domain = util::attribute_to_string(e.attributes(), "domain");
                            let port = util::attribute_to_i64(e.attributes(), "port");
                            let path = util::attribute_to_string(e.attributes(), "path");
                            let register_procedure = util::attribute_to_string(e.attributes(),
                                                                               "registerProcedure");
                            let protocol = util::attribute_to_string(e.attributes(), "protocol");

                            let mut cloud_builder = CloudBuilder::new();
                            cloud_builder.domain(domain.expect("Cloud Domain cannot be None").as_str());
                            cloud_builder.port(port.expect("Cloud Port cannot be None"));
                            cloud_builder.path(path.expect("Cloud Path cannot be None").as_str());
                            cloud_builder.register_procedure(register_procedure.expect("Cloud Regoster Procedure cannot be None").as_str());
                            cloud_builder.protocol(protocol.expect("Cloud Protocol cannot be None")
                                .as_str());

                            channel_builder.cloud(Some(cloud_builder.finalize()));
                        }
                        b"comments" => {
                            name = "comments";
                        }
                        b"copyright" => {
                            name = "copyright";
                        }
                        b"day" => {
                            name = "day";
                        }
                        b"description" => {
                            name = "description";
                        }
                        b"docs" => {
                            name = "docs";
                        }
                        b"enclosure" => {
                            let url = util::attribute_to_string(e.attributes(), "url");
                            let length = util::attribute_to_i64(e.attributes(), "length");
                            let enclosure_type = util::attribute_to_string(e.attributes(), "type");

                            let mut enclosure_builder = EnclosureBuilder::new();
                            enclosure_builder.url(url.expect("Enclosure Url cannot be None").as_str());
                            enclosure_builder.length(length.expect("Enclosure Length cannot be None"));
                            enclosure_builder.enclosure_type(enclosure_type.expect("Enclosure Type cannot be None").as_str());

                            item_builder.enclosure(Some(enclosure_builder.finalize()));
                        }
                        b"generator" => {
                            name = "generator";
                        }
                        b"guid" => {
                            name = "guid";

                            let permalink = util::attribute_to_bool(e.attributes(), "isPermalink");

                            guid_builder = GuidBuilder::new();
                            guid_builder.permalink(permalink);
                        }
                        b"height" => {
                            name = "height";
                        }
                        b"hour" => {
                            name = "hour";
                        }
                        b"language" => {
                            name = "language";
                        }
                        b"lastBuildDate" => {
                            name = "lastBuildDate";
                        }
                        b"link" => {
                            name = "link";
                        }
                        b"managingEditor" => {
                            name = "managingEditor";
                        }
                        b"name" => {
                            name = "name";
                        }
                        b"pubDate" => {
                            name = "pubDate";
                        }
                        b"rating" => {
                            name = "rating";
                        }
                        b"source" => {
                            name = "source";

                            let url = util::attribute_to_string(e.attributes(), "url");

                            source_builder = SourceBuilder::new();
                            source_builder.url(url.expect("Source Url cannot be None").as_str());
                        }
                        b"title" => {
                            name = "title";
                        }
                        b"ttl" => {
                            name = "ttl";
                        }
                        b"url" => {
                            name = "url";
                        }
                        b"webMaster" => {
                            name = "webMaster";
                        }
                        b"width" => {
                            name = "width";
                        }
                        _ => (),
                    }
                }
                Ok(Event::Text(e)) => {
                    match name {
                        "author" => {
                            item_builder.author(Some(e.into_string().unwrap()));
                        }
                        "category" => {
                            category_builder.category(e.into_string().unwrap().as_str());
                        }
                        "comments" => {
                            item_builder.comments(Some(e.into_string().unwrap()));
                        }
                        "copyright" => {
                            channel_builder.copyright(Some(e.into_string().unwrap()));
                        }
                        "day" => {
                            channel_skip_days.push(e.into_string().unwrap());
                        }
                        "description" => {
                            match element {
                                "channel" => {
                                    channel_builder.description(e.into_string().unwrap().as_str());
                                }
                                "image" => {
                                    image_builder.description(Some(e.into_string().unwrap()));
                                }
                                "textInput" => {
                                    text_input_builder.description(e.into_string()
                                        .unwrap()
                                        .as_str());
                                }
                                "item" => {
                                    item_builder.description(Some(e.into_string().unwrap()));
                                }
                                _ => (),
                            };
                        }
                        "docs" => {
                            channel_builder.docs(Some(e.into_string().unwrap()));
                        }
                        "generator" => {
                            channel_builder.generator(Some(e.into_string().unwrap()));
                        }
                        "guid" => {
                            guid_builder.guid(e.into_string().unwrap().as_str());
                        }
                        "height" => {
                            image_builder.height(Some(i64::from_str(e.into_string()
                                    .unwrap()
                                    .as_str())
                                .expect(errors::str_to_i64_error())));
                        }
                        "hour" => {
                            channel_skip_hours.push(i64::from_str(e.into_string()
                                    .unwrap()
                                    .as_str())
                                .expect(errors::str_to_i64_error()));
                        }
                        "language" => {
                            channel_builder.language(Some(e.into_string().unwrap()));
                        }
                        "lastBuildDate" => {
                            channel_builder.last_build_date(Some(e.into_string().unwrap()));
                        }
                        "link" => {
                            match element {
                                "channel" => {
                                    channel_builder.link(e.into_string().unwrap().as_str());
                                }
                                "image" => {
                                    image_builder.link(e.into_string().unwrap().as_str());
                                }
                                "textInput" => {
                                    text_input_builder.link(e.into_string().unwrap().as_str());
                                }
                                "item" => {
                                    item_builder.link(Some(e.into_string().unwrap()));
                                }
                                _ => (),
                            };
                        }
                        "managingEditor" => {
                            channel_builder.managing_editor(Some(e.into_string().unwrap()));
                        }
                        "name" => {
                            text_input_builder.name(e.into_string().unwrap().as_str());
                        }
                        "pubDate" => {
                            match element {
                                "channel" => {
                                    channel_builder.pub_date(Some(e.into_string().unwrap()));
                                }
                                "item" => {
                                    item_builder.pub_date(Some(e.into_string().unwrap()));
                                }
                                _ => (),
                            };
                        }
                        "rating" => {
                            channel_builder.rating(Some(e.into_string().unwrap()));
                        }
                        "source" => {
                            source_builder.source(e.into_string().unwrap().as_str());
                        }
                        "title" => {
                            match element {
                                "channel" => {
                                    channel_builder.title(e.into_string().unwrap().as_str());
                                }
                                "image" => {
                                    image_builder.title(e.into_string().unwrap().as_str());
                                }
                                "textInput" => {
                                    text_input_builder.title(e.into_string().unwrap().as_str());
                                }
                                "item" => {
                                    item_builder.title(Some(e.into_string().unwrap()));
                                }
                                _ => (),
                            };
                        }
                        "ttl" => {
                            channel_builder.ttl(Some(i64::from_str(e.into_string()
                                    .unwrap()
                                    .as_str())
                                .expect(errors::str_to_i64_error())));
                        }
                        "url" => {
                            image_builder.url(e.into_string().unwrap().as_str());
                        }
                        "webMaster" => {
                            channel_builder.web_master(Some(e.into_string().unwrap()));
                        }
                        "width" => {
                            image_builder.width(Some(i64::from_str(e.into_string()
                                    .unwrap()
                                    .as_str())
                                .expect(errors::str_to_i64_error())));
                        }
                        _ => (),
                    };
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"category" => {
                            match element {
                                "channel" => {
                                    channel_categories.push(category_builder.finalize());
                                }
                                "item" => {
                                    item_categories.push(category_builder.finalize());
                                }
                                _ => (),
                            };
                        }
                        b"channel" => {
                            let mut image = None;
                            if image_tag_present {
                                image = Some(image_builder.finalize());
                            }
                            let mut text_input = None;
                            if text_input_tag_present {
                                text_input = Some(text_input_builder.finalize());
                            }

                            channel_builder.categories(Some(channel_categories.clone()));
                            channel_builder.image(image);
                            channel_builder.items(Some(items.clone()));
                            channel_builder.skip_days(Some(channel_skip_days.clone()));
                            channel_builder.skip_hours(Some(channel_skip_hours.clone()));
                            channel_builder.text_input(text_input);
                        }
                        b"item" => {
                            item_builder.categories(Some(item_categories.clone()));
                            items.push(item_builder.finalize());
                        }
                        b"guid" => {
                            item_builder.guid(Some(guid_builder.finalize()));
                        }
                        b"source" => {
                            item_builder.source(Some(source_builder.finalize()));
                        }
                        _ => (),
                    };
                }
                Err((e, pos)) => panic!("{:?} at position {}", e, pos),
                _ => (),
            }
        }

        ChannelReader { channel: channel_builder.finalize() }
    }

    /// Get the `Channel` after parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::ChannelReader;
    ///
    /// let rss = r#"<?xml version="1.0" encoding="UTF-8"?>
    ///             <rss version="2.0">
    ///                  <channel>
    ///                      <title>The Linux Action Show! OGG</title>
    ///                      <link>http://www.jupiterbroadcasting.com</link>
    ///                      <description>Ogg Vorbis audio versions of The Linux Action Show! A show that covers everything geeks care about in the computer industry. Get a solid dose of Linux, gadgets, news events and much more!</description>
    ///                  </channel>
    ///              </rss>"#;
    ///
    /// let feed_reader = ChannelReader::new(rss);
    /// feed_reader.channel();
    /// ```
    pub fn channel(self) -> Channel {
        self.channel.clone()
    }
}
