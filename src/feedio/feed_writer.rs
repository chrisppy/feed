// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `FeedWriter`.

use errors;
use feedio::FeedWriter;
use quick_xml::{Element, XmlWriter};
use quick_xml::Event::*;
use rss::Channel;
use std::io::Cursor;

impl FeedWriter {
    /// Construct a new `FeedWriter` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::{ChannelReader, FeedWriter};
    ///
    /// let feed_reader = ChannelReader::new("String");
    /// let channel = feed_reader.channel();
    ///
    /// let feed_writer = FeedWriter::new(Some(channel));
    /// ```
    pub fn new(channel: Option<Channel>) -> FeedWriter {
        if channel.is_some() {
            let channel = channel.unwrap();
            let mut writer = XmlWriter::new(Cursor::new(Vec::new()));

            let xml_tag_str = "?xml";
            let mut xml_tag = Element::new(xml_tag_str);
            xml_tag.push_attribute(b"version", "1.0");
            xml_tag.push_attribute(b"encoding", "UTF-8");
            writer.write(Start(xml_tag)).expect(errors::tag_start_error(xml_tag_str).as_str());
            writer.write(End(Element::new(xml_tag_str)))
                .expect(errors::tag_end_error(xml_tag_str).as_str());

            let rss_tag_str = "rss";
            let mut rss_tag = Element::new(rss_tag_str);
            rss_tag.push_attribute(b"version", "2.0");
            writer.write(Start(rss_tag)).expect(errors::tag_start_error(rss_tag_str).as_str());

            let channel_tag_str = "channel";
            let channel_tag = Element::new(channel_tag_str);
            writer.write(Start(channel_tag))
                .expect(errors::tag_start_error(channel_tag_str).as_str());

            let title_tag_str = "title";
            let title_tag = Element::new(title_tag_str);
            writer.write(Start(title_tag)).expect(errors::tag_start_error(title_tag_str).as_str());
            writer.write(Text(Element::new(channel.title().as_str())))
                .expect(errors::tag_text_error(title_tag_str).as_str());
            writer.write(End(Element::new(title_tag_str)))
                .expect(errors::tag_end_error(title_tag_str).as_str());

            let link_tag_str = "link";
            let link_tag = Element::new(link_tag_str);
            writer.write(Start(link_tag)).expect(errors::tag_start_error(link_tag_str).as_str());
            writer.write(Text(Element::new(channel.link().as_str())))
                .expect(errors::tag_text_error(link_tag_str).as_str());
            writer.write(End(Element::new(link_tag_str)))
                .expect(errors::tag_end_error(link_tag_str).as_str());

            let description_tag_str = "description";
            let description_tag = Element::new(description_tag_str);
            writer.write(Start(description_tag))
                .expect(errors::tag_start_error(description_tag_str).as_str());
            writer.write(Text(Element::new(channel.description().as_str())))
                .expect(errors::tag_text_error(description_tag_str).as_str());
            writer.write(End(Element::new(description_tag_str)))
                .expect(errors::tag_end_error(description_tag_str).as_str());

            if channel.generator().is_some() {
                let generator_tag_str = "generator";
                let generator_tag = Element::new(generator_tag_str);
                writer.write(Start(generator_tag))
                    .expect(errors::tag_start_error(generator_tag_str).as_str());
                writer.write(Text(Element::new(channel.generator().unwrap().as_str())))
                    .expect(errors::tag_text_error(generator_tag_str).as_str());
                writer.write(End(Element::new(generator_tag_str)))
                    .expect(errors::tag_end_error(generator_tag_str).as_str());
            }

            if channel.docs().is_some() {
                let docs_tag_str = "docs";
                let docs_tag = Element::new(docs_tag_str);
                writer.write(Start(docs_tag))
                    .expect(errors::tag_start_error(docs_tag_str).as_str());
                writer.write(Text(Element::new(channel.docs().unwrap().as_str())))
                    .expect(errors::tag_text_error(docs_tag_str).as_str());
                writer.write(End(Element::new(docs_tag_str)))
                    .expect(errors::tag_end_error(docs_tag_str).as_str());
            }

            if channel.language().is_some() {
                let language_tag_str = "language";
                let language_tag = Element::new(language_tag_str);
                writer.write(Start(language_tag))
                    .expect(errors::tag_start_error(language_tag_str).as_str());
                writer.write(Text(Element::new(channel.language().unwrap().as_str())))
                    .expect(errors::tag_text_error(language_tag_str).as_str());
                writer.write(End(Element::new(language_tag_str)))
                    .expect(errors::tag_end_error(language_tag_str).as_str());
            }

            if channel.copyright().is_some() {
                let copyright_tag_str = "copyright";
                let copyright_tag = Element::new(copyright_tag_str);
                writer.write(Start(copyright_tag))
                    .expect(errors::tag_start_error(copyright_tag_str).as_str());
                writer.write(Text(Element::new(channel.copyright().unwrap().as_str())))
                    .expect(errors::tag_text_error(copyright_tag_str).as_str());
                writer.write(End(Element::new(copyright_tag_str)))
                    .expect(errors::tag_end_error(copyright_tag_str).as_str());
            }

            if channel.managing_editor().is_some() {
                let managing_editor_tag_str = "managingEditor";
                let managing_editor_tag = Element::new(managing_editor_tag_str);
                writer.write(Start(managing_editor_tag))
                    .expect(errors::tag_start_error(managing_editor_tag_str).as_str());
                writer.write(Text(Element::new(channel.managing_editor().unwrap().as_str())))
                    .expect(errors::tag_text_error(managing_editor_tag_str).as_str());
                writer.write(End(Element::new(managing_editor_tag_str)))
                    .expect(errors::tag_end_error(managing_editor_tag_str).as_str());
            }

            if channel.web_master().is_some() {
                let web_master_tag_str = "webMaster";
                let web_master_tag = Element::new(web_master_tag_str);
                writer.write(Start(web_master_tag))
                    .expect(errors::tag_start_error(web_master_tag_str).as_str());
                writer.write(Text(Element::new(channel.web_master().unwrap().as_str())))
                    .expect(errors::tag_text_error(web_master_tag_str).as_str());
                writer.write(End(Element::new(web_master_tag_str)))
                    .expect(errors::tag_end_error(web_master_tag_str).as_str());
            }

            if channel.pub_date().is_some() {
                let pub_date_tag_str = "pubDate";
                let pub_date_tag = Element::new(pub_date_tag_str);
                writer.write(Start(pub_date_tag))
                    .expect(errors::tag_start_error(pub_date_tag_str).as_str());
                writer.write(Text(Element::new(channel.pub_date().unwrap().to_rfc2822())))
                    .expect(errors::tag_text_error(pub_date_tag_str).as_str());
                writer.write(End(Element::new(pub_date_tag_str)))
                    .expect(errors::tag_end_error(pub_date_tag_str).as_str());
            }

            if channel.last_build_date().is_some() {
                let last_build_date_tag_str = "lastBuildDate";
                let last_build_date_tag = Element::new(last_build_date_tag_str);
                writer.write(Start(last_build_date_tag))
                    .expect(errors::tag_start_error(last_build_date_tag_str).as_str());
                writer.write(Text(Element::new(channel.last_build_date().unwrap().to_rfc2822())))
                    .expect(errors::tag_text_error(last_build_date_tag_str).as_str());
                writer.write(End(Element::new(last_build_date_tag_str)))
                    .expect(errors::tag_end_error(last_build_date_tag_str).as_str());
            }

            if channel.cloud().is_some() {
                let cloud_tag_str = "cloud";
                let mut cloud_tag = Element::new(cloud_tag_str);
                cloud_tag.push_attribute(b"domain", channel.cloud().unwrap().domain().as_str());
                cloud_tag.push_attribute(b"port",
                                         channel.cloud().unwrap().port().to_string().as_str());
                cloud_tag.push_attribute(b"path", channel.cloud().unwrap().path().as_str());
                cloud_tag.push_attribute(b"registerProcedure",
                                         channel.cloud().unwrap().register_procedure().as_str());
                cloud_tag.push_attribute(b"protocol", channel.cloud().unwrap().protocol().as_str());
                writer.write(Start(cloud_tag))
                    .expect(errors::tag_start_error(cloud_tag_str).as_str());
                writer.write(End(Element::new(cloud_tag_str)))
                    .expect(errors::tag_end_error(cloud_tag_str).as_str());
            }

            if channel.ttl().is_some() {
                let ttl_tag_str = "ttl";
                let ttl_tag = Element::new(ttl_tag_str);
                writer.write(Start(ttl_tag))
                    .expect(errors::tag_start_error(ttl_tag_str).as_str());
                writer.write(Text(Element::new(channel.ttl().unwrap().to_string().as_str())))
                    .expect(errors::tag_text_error(ttl_tag_str).as_str());
                writer.write(End(Element::new(ttl_tag_str)))
                    .expect(errors::tag_end_error(ttl_tag_str).as_str());
            }

            if channel.categories().is_some() {
                for category in channel.categories().unwrap() {
                    let category_tag_str = "category";
                    let mut category_tag = Element::new(category_tag_str);
                    if category.domain().is_some() {
                        category_tag.push_attribute(b"domain", category.domain().unwrap().as_str());
                    }
                    writer.write(Start(category_tag))
                        .expect(errors::tag_start_error(category_tag_str).as_str());
                    writer.write(Text(Element::new(category.category())))
                        .expect(errors::tag_text_error(category_tag_str).as_str());
                    writer.write(End(Element::new(category_tag_str)))
                        .expect(errors::tag_end_error(category_tag_str).as_str());
                }
            }

            if channel.image().is_some() {
                let image_tag_str = "image";
                let image_tag = Element::new(image_tag_str);
                writer.write(Start(image_tag))
                    .expect(errors::tag_start_error(image_tag_str).as_str());

                let image_link_tag_str = "link";
                let image_link_tag = Element::new(image_link_tag_str);
                writer.write(Start(image_link_tag))
                    .expect(errors::tag_start_error(image_link_tag_str).as_str());
                writer.write(Text(Element::new(channel.image()
                        .unwrap()
                        .link()
                        .as_str())))
                    .expect(errors::tag_text_error(image_link_tag_str).as_str());
                writer.write(End(Element::new(image_link_tag_str)))
                    .expect(errors::tag_end_error(image_link_tag_str).as_str());

                let image_url_tag_str = "url";
                let image_url_tag = Element::new(image_url_tag_str);
                writer.write(Start(image_url_tag))
                    .expect(errors::tag_start_error(image_url_tag_str).as_str());
                writer.write(Text(Element::new(channel.image()
                        .unwrap()
                        .url()
                        .as_str())))
                    .expect(errors::tag_text_error(image_url_tag_str).as_str());
                writer.write(End(Element::new(image_url_tag_str)))
                    .expect(errors::tag_end_error(image_url_tag_str).as_str());

                let image_title_tag_str = "title";
                let image_title_tag = Element::new(image_title_tag_str);
                writer.write(Start(image_title_tag))
                    .expect(errors::tag_start_error(image_title_tag_str).as_str());
                writer.write(Text(Element::new(channel.image()
                        .unwrap()
                        .title()
                        .as_str())))
                    .expect(errors::tag_text_error(image_title_tag_str).as_str());
                writer.write(End(Element::new(image_title_tag_str)))
                    .expect(errors::tag_end_error(image_title_tag_str).as_str());

                let image_width_tag_str = "width";
                let image_width_tag = Element::new(image_width_tag_str);
                writer.write(Start(image_width_tag))
                    .expect(errors::tag_start_error(image_width_tag_str).as_str());
                writer.write(Text(Element::new(channel.image()
                        .unwrap()
                        .width()
                        .to_string()
                        .as_str())))
                    .expect(errors::tag_text_error(image_width_tag_str).as_str());
                writer.write(End(Element::new(image_width_tag_str)))
                    .expect(errors::tag_end_error(image_width_tag_str).as_str());

                let image_height_tag_str = "height";
                let image_height_tag = Element::new(image_height_tag_str);
                writer.write(Start(image_height_tag))
                    .expect(errors::tag_start_error(image_height_tag_str).as_str());
                writer.write(Text(Element::new(channel.image()
                        .unwrap()
                        .height()
                        .to_string()
                        .as_str())))
                    .expect(errors::tag_text_error(image_height_tag_str).as_str());
                writer.write(End(Element::new(image_height_tag_str)))
                    .expect(errors::tag_end_error(image_height_tag_str).as_str());

                if channel.image().unwrap().description().is_some() {
                    let image_description_tag_str = "description";
                    let image_description_tag = Element::new(image_description_tag_str);
                    writer.write(Start(image_description_tag))
                        .expect(errors::tag_start_error(image_description_tag_str).as_str());
                    writer.write(Text(Element::new(channel.image()
                            .unwrap()
                            .description()
                            .unwrap()
                            .as_str())))
                        .expect(errors::tag_text_error(image_description_tag_str).as_str());
                    writer.write(End(Element::new(image_description_tag_str)))
                        .expect(errors::tag_end_error(image_description_tag_str).as_str());
                }

                writer.write(End(Element::new(image_tag_str)))
                    .expect(errors::tag_end_error(image_tag_str).as_str());
            }

            if channel.rating().is_some() {
                let rating_tag_str = "rating";
                let rating_tag = Element::new(rating_tag_str);
                writer.write(Start(rating_tag))
                    .expect(errors::tag_start_error(rating_tag_str).as_str());
                writer.write(Text(Element::new(channel.rating().unwrap().as_str())))
                    .expect(errors::tag_text_error(rating_tag_str).as_str());
                writer.write(End(Element::new(rating_tag_str)))
                    .expect(errors::tag_end_error(rating_tag_str).as_str());
            }

            if channel.text_input().is_some() {
                let text_input_str = "textInput";
                let text_input = Element::new(text_input_str);
                writer.write(Start(text_input))
                    .expect(errors::tag_start_error(text_input_str).as_str());

                let title_tag_str = "title";
                let title_tag = Element::new(title_tag_str);
                writer.write(Start(title_tag))
                    .expect(errors::tag_start_error(title_tag_str).as_str());
                writer.write(Text(Element::new(channel.text_input()
                        .unwrap()
                        .title()
                        .as_str())))
                    .expect(errors::tag_text_error(title_tag_str).as_str());
                writer.write(End(Element::new(title_tag_str)))
                    .expect(errors::tag_end_error(title_tag_str).as_str());

                let description_tag_str = "description";
                let description_tag = Element::new(description_tag_str);
                writer.write(Start(description_tag))
                    .expect(errors::tag_start_error(description_tag_str).as_str());
                writer.write(Text(Element::new(channel.description().as_str())))
                    .expect(errors::tag_text_error(description_tag_str).as_str());
                writer.write(End(Element::new(description_tag_str)))
                    .expect(errors::tag_end_error(description_tag_str).as_str());

                let name_tag_str = "name";
                let name_tag = Element::new(name_tag_str);
                writer.write(Start(name_tag))
                    .expect(errors::tag_start_error(name_tag_str).as_str());
                writer.write(Text(Element::new(channel.text_input()
                        .unwrap()
                        .name()
                        .as_str())))
                    .expect(errors::tag_text_error(name_tag_str).as_str());
                writer.write(End(Element::new(name_tag_str)))
                    .expect(errors::tag_end_error(name_tag_str).as_str());

                let link_tag_str = "link";
                let link_tag = Element::new(link_tag_str);
                writer.write(Start(link_tag))
                    .expect(errors::tag_start_error(link_tag_str).as_str());
                writer.write(Text(Element::new(channel.text_input()
                        .unwrap()
                        .link()
                        .as_str())))
                    .expect(errors::tag_text_error(link_tag_str).as_str());
                writer.write(End(Element::new(link_tag_str)))
                    .expect(errors::tag_end_error(link_tag_str).as_str());

                writer.write(End(Element::new(text_input_str)))
                    .expect(errors::tag_end_error(text_input_str).as_str());
            }

            if channel.skip_hours().is_some() {
                let skip_hours_tag_str = "skipHours";
                let skip_hours_tag = Element::new(skip_hours_tag_str);
                writer.write(Start(skip_hours_tag))
                    .expect(errors::tag_start_error(skip_hours_tag_str).as_str());
                for hour in channel.skip_hours().unwrap() {
                    let hour_tag_str = "hour";
                    let hour_tag = Element::new(hour_tag_str);
                    writer.write(Start(hour_tag))
                        .expect(errors::tag_start_error(hour_tag_str).as_str());
                    writer.write(Text(Element::new(hour.to_string().as_str())))
                        .expect(errors::tag_text_error(hour_tag_str).as_str());
                    writer.write(End(Element::new(hour_tag_str)))
                        .expect(errors::tag_end_error(hour_tag_str).as_str());
                }
                writer.write(End(Element::new(skip_hours_tag_str)))
                    .expect(errors::tag_end_error(skip_hours_tag_str).as_str());
            }

            if channel.skip_days().is_some() {
                let skip_days_tag_str = "skipDays";
                let skip_days_tag = Element::new(skip_days_tag_str);
                writer.write(Start(skip_days_tag))
                    .expect(errors::tag_start_error(skip_days_tag_str).as_str());
                for day in channel.skip_days().unwrap() {
                    let day_tag_str = "day";
                    let day_tag = Element::new(day_tag_str);
                    writer.write(Start(day_tag))
                        .expect(errors::tag_start_error(day_tag_str).as_str());
                    writer.write(Text(Element::new(day.as_str())))
                        .expect(errors::tag_text_error(day_tag_str).as_str());
                    writer.write(End(Element::new(day_tag_str)))
                        .expect(errors::tag_end_error(day_tag_str).as_str());
                }
                writer.write(End(Element::new(skip_days_tag_str)))
                    .expect(errors::tag_end_error(skip_days_tag_str).as_str());
            }

            if channel.items().is_some() {
                for item in channel.items().unwrap() {
                    let item_tag_str = "item";
                    let item_tag = Element::new(item_tag_str);
                    writer.write(Start(item_tag))
                        .expect(errors::tag_start_error(item_tag_str).as_str());

                    if item.title().is_some() {
                        let item_title_tag_str = "title";
                        let item_title_tag = Element::new(item_title_tag_str);
                        writer.write(Start(item_title_tag))
                            .expect(errors::tag_start_error(item_title_tag_str).as_str());
                        writer.write(Text(Element::new(item.title()
                                .unwrap()
                                .as_str())))
                            .expect(errors::tag_text_error(item_title_tag_str).as_str());
                        writer.write(End(Element::new(item_title_tag_str)))
                            .expect(errors::tag_end_error(item_title_tag_str).as_str());
                    }

                    if item.link().is_some() {
                        let item_link_tag_str = "link";
                        let item_link_tag = Element::new(item_link_tag_str);
                        writer.write(Start(item_link_tag))
                            .expect(errors::tag_start_error(item_link_tag_str).as_str());
                        writer.write(Text(Element::new(item.link()
                                .unwrap()
                                .as_str())))
                            .expect(errors::tag_text_error(item_link_tag_str).as_str());
                        writer.write(End(Element::new(item_link_tag_str)))
                            .expect(errors::tag_end_error(item_link_tag_str).as_str());
                    }

                    if item.description().is_some() {
                        let item_description_tag_str = "description";
                        let item_description_tag = Element::new(item_description_tag_str);
                        writer.write(Start(item_description_tag))
                            .expect(errors::tag_start_error(item_description_tag_str).as_str());
                        writer.write(Text(Element::new(item.description()
                                .unwrap()
                                .as_str())))
                            .expect(errors::tag_text_error(item_description_tag_str).as_str());
                        writer.write(End(Element::new(item_description_tag_str)))
                            .expect(errors::tag_end_error(item_description_tag_str).as_str());
                    }

                    if item.author().is_some() {
                        let item_author_tag_str = "author";
                        let item_author_tag = Element::new(item_author_tag_str);
                        writer.write(Start(item_author_tag))
                            .expect(errors::tag_start_error(item_author_tag_str).as_str());
                        writer.write(Text(Element::new(item.author()
                                .unwrap()
                                .as_str())))
                            .expect(errors::tag_text_error(item_author_tag_str).as_str());
                        writer.write(End(Element::new(item_author_tag_str)))
                            .expect(errors::tag_end_error(item_author_tag_str).as_str());
                    }

                    if item.categories().is_some() {
                        for category in item.categories().unwrap() {
                            let item_category_tag_str = "category";
                            let mut item_category_tag = Element::new(item_category_tag_str);
                            if category.domain().is_some() {
                                item_category_tag.push_attribute(b"domain",
                                                                 category.domain()
                                                                     .unwrap()
                                                                     .as_str());
                            }
                            writer.write(Start(item_category_tag))
                                .expect(errors::tag_start_error(item_category_tag_str).as_str());
                            writer.write(Text(Element::new(category.category())))
                                .expect(errors::tag_text_error(item_category_tag_str).as_str());
                            writer.write(End(Element::new(item_category_tag_str)))
                                .expect(errors::tag_end_error(item_category_tag_str).as_str());
                        }
                    }

                    if item.comments().is_some() {
                        let item_comments_tag_str = "comments";
                        let item_comments_tag = Element::new(item_comments_tag_str);
                        writer.write(Start(item_comments_tag))
                            .expect(errors::tag_start_error(item_comments_tag_str).as_str());
                        writer.write(Text(Element::new(item.comments()
                                .unwrap()
                                .as_str())))
                            .expect(errors::tag_text_error(item_comments_tag_str).as_str());
                        writer.write(End(Element::new(item_comments_tag_str)))
                            .expect(errors::tag_end_error(item_comments_tag_str).as_str());
                    }

                    if item.enclosure().is_some() {
                        let item_enclosure_tag_str = "enclosure";
                        let mut item_enclosure_tag = Element::new(item_enclosure_tag_str);
                        item_enclosure_tag.push_attribute(b"url",
                                                          item.enclosure()
                                                              .unwrap()
                                                              .url()
                                                              .as_str());
                        item_enclosure_tag.push_attribute(b"length",
                                                          item.enclosure()
                                                              .unwrap()
                                                              .length()
                                                              .to_string()
                                                              .as_str());
                        item_enclosure_tag.push_attribute(b"type",
                                                          item.enclosure()
                                                              .unwrap()
                                                              .enclosure_type()
                                                              .as_str());
                        writer.write(Start(item_enclosure_tag))
                            .expect(errors::tag_start_error(item_enclosure_tag_str).as_str());

                    }

                    if item.guid().is_some() {
                        let item_guid_tag_str = "guid";
                        let mut item_guid_tag = Element::new(item_guid_tag_str);
                        item_guid_tag.push_attribute(b"isPermaLink",
                                                     item.guid()
                                                         .unwrap()
                                                         .permalink()
                                                         .to_string()
                                                         .as_str());
                        writer.write(Start(item_guid_tag))
                            .expect(errors::tag_start_error(item_guid_tag_str).as_str());
                        writer.write(Text(Element::new(item.guid().unwrap().guid().as_str())))
                            .expect(errors::tag_text_error(item_guid_tag_str).as_str());
                        writer.write(End(Element::new(item_guid_tag_str)))
                            .expect(errors::tag_end_error(item_guid_tag_str).as_str());
                    }

                    if item.pub_date().is_some() {
                        let item_pub_date_tag_str = "pubDate";
                        let item_pub_date_tag = Element::new(item_pub_date_tag_str);
                        writer.write(Start(item_pub_date_tag))
                            .expect(errors::tag_start_error(item_pub_date_tag_str).as_str());
                        writer.write(Text(Element::new(channel.pub_date().unwrap().to_rfc2822())))
                            .expect(errors::tag_text_error(item_pub_date_tag_str).as_str());
                        writer.write(End(Element::new(item_pub_date_tag_str)))
                            .expect(errors::tag_end_error(item_pub_date_tag_str).as_str());
                    }

                    if item.source().is_some() {
                        let item_source_tag_str = "source";
                        let mut item_source_tag = Element::new(item_source_tag_str);
                        item_source_tag.push_attribute(b"url",
                                                       item.source()
                                                           .unwrap()
                                                           .url()
                                                           .to_string()
                                                           .as_str());
                        writer.write(Start(item_source_tag))
                            .expect(errors::tag_start_error(item_source_tag_str).as_str());
                        writer.write(Text(Element::new(item.source().unwrap().source().as_str())))
                            .expect(errors::tag_text_error(item_source_tag_str).as_str());
                        writer.write(End(Element::new(item_source_tag_str)))
                            .expect(errors::tag_end_error(item_source_tag_str).as_str());
                    }

                    writer.write(End(Element::new(item_tag_str)))
                        .expect(errors::tag_end_error(item_tag_str).as_str());
                }
            }

            writer.write(End(Element::new(channel_tag_str)))
                .expect(errors::tag_end_error(channel_tag_str).as_str());
            writer.write(End(Element::new(rss_tag_str)))
                .expect(errors::tag_end_error(rss_tag_str).as_str());
            FeedWriter { xml: writer.into_inner().into_inner() }
        } else {
            panic!("There must be a value for Channel and/or AtomFeed");
        }
    }


    /// Convert the `Channel` to XML.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::{ChannelReader, FeedWriter};
    ///
    /// let feed_reader = ChannelReader::new("String");
    /// let channel = feed_reader.channel();
    ///
    /// let xml = FeedWriter::new(Some(channel)).xml();
    /// ```
    pub fn xml(&self) -> Vec<u8> {
        self.xml.clone()
    }
}
