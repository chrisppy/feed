// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod util;

use quick_xml::{XmlReader, Event};
use rss::{Channel, ChannelBuilder};
use rss::category::{Category, CategoryBuilder};
use rss::cloud::CloudBuilder;
use rss::enclosure::EnclosureBuilder;
use rss::guid::GuidBuilder;
use rss::image::ImageBuilder;
use rss::item::{Item, ItemBuilder};
use rss::source::SourceBuilder;
use rss::text_input::TextInputBuilder;

pub struct FeedReader {
    channel: Channel,
}


impl FeedReader {
    pub fn new(feed: Option<String>) -> FeedReader {
        if feed.is_none() {
            panic!("The feed is empty!");
        }
        let mut channel_builder = ChannelBuilder::new();
        let mut image_builder = ImageBuilder::new();
        let mut item_builder = ItemBuilder::new();
        let mut text_input_builder = TextInputBuilder::new();

        let mut channel_categories: Vec<Category> = Vec::new();
        let mut channel_skip_days: Vec<String> = Vec::new();
        let mut channel_skip_hours: Vec<i64> = Vec::new();
        let mut items: Vec<Item> = Vec::new();
        let mut item_categories: Vec<Category> = Vec::new();
        let mut element = "channel";

        let reader = XmlReader::from_str(feed.as_ref().unwrap()).trim_text(true);
        for r in reader {
            match r {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        //TODO: change element based on the element it is on to determine for example url
                        b"image"          => {
                            element = "image";
                        },
                        b"textInput"      => {
                            element = "textInput";
                        },
                        b"item"           => {
                            item_builder = ItemBuilder::new();
                            element = "item";
                        },
                        b"author"         => {
                            let author = util::content_to_option_string(e.content());
                            item_builder.author(author);
                        },
                        b"category"       => {
                            let mut category_builder = CategoryBuilder::new();

                            let category = util::content_to_str(e.content());
                            category_builder.category(category);

                            let domain = util::attribute_to_option_string(e.attributes(), 0);
                            category_builder.domain(domain);

                            match element {
                                "channel" => {channel_categories.push(category_builder.finalize());},
                                "item"    => {item_categories.push(category_builder.finalize());},
                                _         => (),
                            }

                        },
                        b"cloud"          => {
                            let mut cloud_builder = CloudBuilder::new();

                            let domain = util::attribute_to_str(e.attributes(), 0);
                            cloud_builder.domain(domain);

                            let port = util::attribute_to_i64(e.attributes(), 1);
                            cloud_builder.port(port);

                            let path = util::attribute_to_str(e.attributes(), 2);
                            cloud_builder.path(path);

                            let register_procedure = util::attribute_to_str(e.attributes(), 3);
                            cloud_builder.register_procedure(register_procedure);

                            let protocol = util::attribute_to_str(e.attributes(), 4);
                            cloud_builder.protocol(protocol);

                            channel_builder.cloud(Some(cloud_builder.finalize()));
                        },
                        b"comments"       => {
                            let comments = util::content_to_option_string(e.content());
                            item_builder.comments(comments);
                        }
                        b"copyright"      => {
                            let copyright = util::content_to_option_string(e.content());
                            channel_builder.copyright(copyright);
                        },
                        b"description"    => {
                            let description = util::content_to_str(e.content());
                            match element {
                                "channel"   => {channel_builder.description(description);},
                                "image"     => {image_builder.description(util::str_to_option_string(description));},
                                "textInput" => {text_input_builder.description(description);},
                                "item"      => {item_builder.description(util::str_to_option_string(description));},
                                _           => (),
                            };
                        },
                        b"docs"           => {
                            let docs = util::content_to_option_string(e.content());
                            channel_builder.docs(docs);
                        },
                        b"enclosure"      => {
                            let mut enclosure_builder = EnclosureBuilder::new();

                            let url = util::attribute_to_str(e.attributes(), 0);
                            enclosure_builder.url(url);

                            let length = util::attribute_to_i64(e.attributes(), 1);
                            enclosure_builder.length(length);

                            let enclosure_type = util::attribute_to_str(e.attributes(), 2);
                            enclosure_builder.enclosure_type(enclosure_type);

                            item_builder.enclosure(Some(enclosure_builder.finalize()));
                        },
                        b"generator"      => {
                            let generator = util::content_to_option_string(e.content());
                            channel_builder.generator(generator);
                        },
                        b"guid"           => {
                            let mut guid_builder = GuidBuilder::new();

                            let guid = util::content_to_str(e.content());
                            guid_builder.guid(guid);

                            let is_permalink = util::attribute_to_option_bool(e.attributes(), 0);
                            guid_builder.is_permalink(is_permalink);

                            item_builder.guid(Some(guid_builder.finalize()));
                        },
                        b"height"         => {
                            let height = util::content_to_option_i64(e.content());
                            image_builder.height(height);
                        },
                        b"language"       => {
                            let language = util::content_to_option_string(e.content());
                            channel_builder.language(language);
                        },
                        b"lastBuildDate"   => {
                            let last_build_date = util::content_to_option_date(e.content());
                            channel_builder.last_build_date(last_build_date);
                        },
                        b"link"           => {
                            let link = util::content_to_str(e.content());
                            match element {
                                "channel"   => {channel_builder.link(link);},
                                "image"     => {image_builder.link(link);},
                                "textInput" => {text_input_builder.link(link);},
                                "item"      => {item_builder.link(util::str_to_option_string(link));},
                                _           => (),
                            };
                        },
                        b"managingEditor" => {
                            let managing_editor = util::content_to_option_string(e.content());
                            channel_builder.managing_editor(managing_editor);
                        },
                        b"name"           => {
                            let name = util::content_to_str(e.content());
                            text_input_builder.name(name);
                        }
                        b"pubDate"        => {
                            let pub_date = util::content_to_option_date(e.content());
                            match element {
                                "channel" => {channel_builder.pub_date(pub_date);},
                                "item"    => {item_builder.pub_date(pub_date);},
                                _         => (),
                            };
                        },
                        b"rating"         => {
                            let rating = util::content_to_option_string(e.content());
                            channel_builder.rating(rating);
                        },
                        b"skipDays"       => {
                            let skip_day = util::content_to_str(e.content());
                            channel_skip_days.push(skip_day.to_string());
                        },
                        b"skipHours"      => {
                            let skip_hour = util::content_to_i64(e.content());
                            channel_skip_hours.push(skip_hour);
                        },
                        b"source"         => {
                            let mut source_builder = SourceBuilder::new();

                            let source = util::content_to_str(e.content());
                            source_builder.source(source);

                            let url = util::attribute_to_str(e.attributes(), 0);
                            source_builder.url(url);

                            item_builder.source(Some(source_builder.finalize()));
                        },
                        b"title"          => {
                            let title = util::content_to_str(e.content());
                            match element {
                                "channel"   => {channel_builder.title(title);},
                                "image"     => {image_builder.title(title);},
                                "textInput" => {text_input_builder.title(title);},
                                "item"      => {item_builder.title(util::str_to_option_string(title));}
                                _           => (),
                            };
                        },
                        b"ttl"            => {
                            let ttl = util::content_to_option_i64(e.content());
                            channel_builder.ttl(ttl);
                        },
                        b"url"            => {
                            let url = util::content_to_str(e.content());
                            image_builder.url(url);
                        },
                        b"webMaster"      => {
                            let web_master = util::content_to_option_string(e.content());
                            channel_builder.web_master(web_master);
                        },
                        b"width"          => {
                            let width = util::content_to_option_i64(e.content());
                            image_builder.width(width);
                        },
                        _                 => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"item"           => {
                            item_builder.categories(Some(item_categories.clone()));
                            items.push(item_builder.finalize());
                        },
                        b"channel"        => {
                            channel_builder.categories(Some(channel_categories.clone()));
                            channel_builder.image(Some(image_builder.finalize()));
                            channel_builder.items(Some(items.clone()));
                            channel_builder.skip_days(Some(channel_skip_days.clone()));
                            channel_builder.skip_hours(Some(channel_skip_hours.clone()));
                            channel_builder.text_input(Some(text_input_builder.finalize()));
                        }
                        _                 => (),
                    };
                }
                Err(e)                  => panic!("{:?}", e),
                _                       => (),
            }
        }

        FeedReader {
            channel: channel_builder.finalize(),
        }
    }


    pub fn channel(self) -> Channel {
        self.channel.clone()
    }
}


pub struct FeedWriter {
    pub xml: String,
}


impl FeedWriter {
    pub fn new(channel: Channel) -> FeedWriter {
        FeedWriter {
            xml: String::new(),
        }
    }


    pub fn xml(&self) -> String {
        panic!("Not yet Implemented!");
        self.xml.clone()
    }
}
