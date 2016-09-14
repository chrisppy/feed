// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! Implementation of `FeedWriter`.

use atom::{AtomFeed, Entry};
use errors;
use feedio::FeedWriter;
use quick_xml::{Element, XmlWriter};
use quick_xml::Event::*;
use rss::{Channel, Item};
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
    /// FeedWriter::new(channel, None);
    /// ```
    pub fn new(channel: Channel, feed: Option<AtomFeed>) -> FeedWriter {
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
        rss_tag.push_attribute(b"xmlns:atom", "http://www.w3.org/2005/Atom");
        rss_tag.push_attribute(b"xmlns:itunes",
                               "http://www.itunes.com/dtds/podcast-1.0.dtd");
        rss_tag.push_attribute(b"version", "2.0");
        writer.write(Start(rss_tag)).expect(errors::tag_start_error(rss_tag_str).as_str());

        write_channel(writer.clone(), channel.clone(), feed.clone());

        writer.write(End(Element::new(rss_tag_str)))
            .expect(errors::tag_end_error(rss_tag_str).as_str());
        FeedWriter { xml: writer.into_inner().into_inner() }
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
    /// FeedWriter::new(channel, None).xml();
    /// ```
    pub fn xml(&self) -> Vec<u8> {
        self.xml.clone()
    }
}


fn write_channel(mut writer: XmlWriter<Cursor<Vec<u8>>>,
                 channel: Channel,
                 feed: Option<AtomFeed>) {
    let channel_tag_str = "channel";
    let channel_tag = Element::new(channel_tag_str);
    writer.write(Start(channel_tag))
        .expect(errors::tag_start_error(channel_tag_str).as_str());

    write_title(writer.clone(), channel.clone());
    write_link(writer.clone(), channel.clone());
    write_description(writer.clone(), channel.clone());
    write_generator(writer.clone(), channel.clone());
    write_docs(writer.clone(), channel.clone());
    write_language(writer.clone(), channel.clone());
    write_copyright(writer.clone(), channel.clone());
    write_managing_editor(writer.clone(), channel.clone());
    write_web_master(writer.clone(), channel.clone());
    write_cloud(writer.clone(), channel.clone());
    write_ttl(writer.clone(), channel.clone());
    write_categories(writer.clone(), channel.clone());
    write_rating(writer.clone(), channel.clone());
    write_pub_date(writer.clone(), channel.clone());
    write_last_build_date(writer.clone(), channel.clone());
    write_image(writer.clone(), channel.clone());
    write_text_input(writer.clone(), channel.clone());
    write_skip_hours(writer.clone(), channel.clone());
    write_skip_days(writer.clone(), channel.clone());
    write_atom_feed(writer.clone(), feed.clone());
    write_items(writer.clone(), channel.clone(), feed.clone());

    writer.write(End(Element::new(channel_tag_str)))
        .expect(errors::tag_end_error(channel_tag_str).as_str());
}


fn write_title(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
    let title_tag_str = "title";
    let title_tag = Element::new(title_tag_str);
    writer.write(Start(title_tag)).expect(errors::tag_start_error(title_tag_str).as_str());
    writer.write(Text(Element::new(channel.title().as_str())))
        .expect(errors::tag_text_error(title_tag_str).as_str());
    writer.write(End(Element::new(title_tag_str)))
        .expect(errors::tag_end_error(title_tag_str).as_str());
}


fn write_link(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
    let link_tag_str = "link";
    let link_tag = Element::new(link_tag_str);
    writer.write(Start(link_tag)).expect(errors::tag_start_error(link_tag_str).as_str());
    writer.write(Text(Element::new(channel.link().as_str())))
        .expect(errors::tag_text_error(link_tag_str).as_str());
    writer.write(End(Element::new(link_tag_str)))
        .expect(errors::tag_end_error(link_tag_str).as_str());
}


fn write_description(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
    let description_tag_str = "description";
    let description_tag = Element::new(description_tag_str);
    writer.write(Start(description_tag))
        .expect(errors::tag_start_error(description_tag_str).as_str());
    writer.write(Text(Element::new(channel.description().as_str())))
        .expect(errors::tag_text_error(description_tag_str).as_str());
    writer.write(End(Element::new(description_tag_str)))
        .expect(errors::tag_end_error(description_tag_str).as_str());
}


fn write_generator(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_docs(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_language(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_copyright(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_managing_editor(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_web_master(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_cloud(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_ttl(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_categories(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_rating(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_pub_date(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_last_build_date(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_image(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}

fn write_text_input(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
    if channel.text_input().is_some() {
        let text_input_str = "textInput";
        let text_input = Element::new(text_input_str);
        writer.write(Start(text_input))
            .expect(errors::tag_start_error(text_input_str).as_str());

        let title_tag_str = "title";
        let title_tag = Element::new(title_tag_str);
        writer.write(Start(title_tag))
            .expect(errors::tag_start_error(title_tag_str).as_str());
        writer.write(Text(Element::new(channel.text_input().unwrap().title().as_str())))
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
        writer.write(Text(Element::new(channel.text_input().unwrap().name().as_str())))
            .expect(errors::tag_text_error(name_tag_str).as_str());
        writer.write(End(Element::new(name_tag_str)))
            .expect(errors::tag_end_error(name_tag_str).as_str());

        let link_tag_str = "link";
        let link_tag = Element::new(link_tag_str);
        writer.write(Start(link_tag))
            .expect(errors::tag_start_error(link_tag_str).as_str());
        writer.write(Text(Element::new(channel.text_input().unwrap().link().as_str())))
            .expect(errors::tag_text_error(link_tag_str).as_str());
        writer.write(End(Element::new(link_tag_str)))
            .expect(errors::tag_end_error(link_tag_str).as_str());

        writer.write(End(Element::new(text_input_str)))
            .expect(errors::tag_end_error(text_input_str).as_str());
    }
}


fn write_skip_hours(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_skip_days(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel) {
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
}


fn write_items(mut writer: XmlWriter<Cursor<Vec<u8>>>, channel: Channel, feed: Option<AtomFeed>) {
    if channel.items().is_some() {
        for item in channel.items().unwrap() {
            let item_tag_str = "item";
            let item_tag = Element::new(item_tag_str);
            writer.write(Start(item_tag))
                .expect(errors::tag_start_error(item_tag_str).as_str());

            write_item_title(writer.clone(), item.clone());
            write_item_link(writer.clone(), item.clone());
            write_item_description(writer.clone(), item.clone());
            write_item_author(writer.clone(), item.clone());
            write_item_categories(writer.clone(), item.clone());
            write_item_comments(writer.clone(), item.clone());
            write_item_enclosure(writer.clone(), item.clone());
            write_item_guid(writer.clone(), item.clone());
            write_item_pub_date(writer.clone(), item.clone());
            write_item_source(writer.clone(), item.clone());

            writer.write(End(Element::new(item_tag_str)))
                .expect(errors::tag_end_error(item_tag_str).as_str());
        }
    }
    if feed.is_some() {
        let entries = feed.clone().unwrap().entries();
        if entries.is_some() {
            for entry in entries.unwrap() {
                let entry_tag_str = "atom:entry";
                let entry_tag = Element::new(entry_tag_str);
                writer.write(Start(entry_tag))
                    .expect(errors::tag_start_error(entry_tag_str).as_str());

                write_feed_entry_id(writer.clone(), entry.clone());
                write_feed_entry_title(writer.clone(), entry.clone());
                write_feed_entry_updated(writer.clone(), entry.clone());
                write_feed_entry_authors(writer.clone(), entry.clone());
                write_feed_entry_content(writer.clone(), entry.clone());
                write_feed_entry_links(writer.clone(), entry.clone());
                write_feed_entry_summary(writer.clone(), entry.clone());
                write_feed_entry_categories(writer.clone(), entry.clone());
                write_feed_entry_contributors(writer.clone(), entry.clone());
                write_feed_entry_published(writer.clone(), entry.clone());
                write_feed_entry_source(writer.clone(), entry.clone());
                write_feed_entry_rights(writer.clone(), entry.clone());

                writer.write(End(Element::new(entry_tag_str)))
                    .expect(errors::tag_end_error(entry_tag_str).as_str());
            }
        }
    }
}


fn write_item_title(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.title().is_some() {
        let item_title_tag_str = "title";
        let item_title_tag = Element::new(item_title_tag_str);
        writer.write(Start(item_title_tag))
            .expect(errors::tag_start_error(item_title_tag_str).as_str());
        writer.write(Text(Element::new(item.title().unwrap().as_str())))
            .expect(errors::tag_text_error(item_title_tag_str).as_str());
        writer.write(End(Element::new(item_title_tag_str)))
            .expect(errors::tag_end_error(item_title_tag_str).as_str());
    }
}


fn write_item_link(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.link().is_some() {
        let item_link_tag_str = "link";
        let item_link_tag = Element::new(item_link_tag_str);
        writer.write(Start(item_link_tag))
            .expect(errors::tag_start_error(item_link_tag_str).as_str());
        writer.write(Text(Element::new(item.link().unwrap().as_str())))
            .expect(errors::tag_text_error(item_link_tag_str).as_str());
        writer.write(End(Element::new(item_link_tag_str)))
            .expect(errors::tag_end_error(item_link_tag_str).as_str());
    }
}


fn write_item_description(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.description().is_some() {
        let item_description_tag_str = "description";
        let item_description_tag = Element::new(item_description_tag_str);
        writer.write(Start(item_description_tag))
            .expect(errors::tag_start_error(item_description_tag_str).as_str());
        writer.write(Text(Element::new(item.description().unwrap().as_str())))
            .expect(errors::tag_text_error(item_description_tag_str).as_str());
        writer.write(End(Element::new(item_description_tag_str)))
            .expect(errors::tag_end_error(item_description_tag_str).as_str());
    }
}


fn write_item_author(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.author().is_some() {
        let item_author_tag_str = "author";
        let item_author_tag = Element::new(item_author_tag_str);
        writer.write(Start(item_author_tag))
            .expect(errors::tag_start_error(item_author_tag_str).as_str());
        writer.write(Text(Element::new(item.author().unwrap().as_str())))
            .expect(errors::tag_text_error(item_author_tag_str).as_str());
        writer.write(End(Element::new(item_author_tag_str)))
            .expect(errors::tag_end_error(item_author_tag_str).as_str());
    }
}


fn write_item_categories(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.categories().is_some() {
        for category in item.categories().unwrap() {
            let item_category_tag_str = "category";
            let mut item_category_tag = Element::new(item_category_tag_str);
            if category.domain().is_some() {
                item_category_tag.push_attribute(b"domain", category.domain().unwrap().as_str());
            }
            writer.write(Start(item_category_tag))
                .expect(errors::tag_start_error(item_category_tag_str).as_str());
            writer.write(Text(Element::new(category.category())))
                .expect(errors::tag_text_error(item_category_tag_str).as_str());
            writer.write(End(Element::new(item_category_tag_str)))
                .expect(errors::tag_end_error(item_category_tag_str).as_str());
        }
    }
}


fn write_item_comments(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.comments().is_some() {
        let item_comments_tag_str = "comments";
        let item_comments_tag = Element::new(item_comments_tag_str);
        writer.write(Start(item_comments_tag))
            .expect(errors::tag_start_error(item_comments_tag_str).as_str());
        writer.write(Text(Element::new(item.comments().unwrap().as_str())))
            .expect(errors::tag_text_error(item_comments_tag_str).as_str());
        writer.write(End(Element::new(item_comments_tag_str)))
            .expect(errors::tag_end_error(item_comments_tag_str).as_str());
    }
}


fn write_item_enclosure(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.enclosure().is_some() {
        let item_enclosure_tag_str = "enclosure";
        let mut item_enclosure_tag = Element::new(item_enclosure_tag_str);
        item_enclosure_tag.push_attribute(b"url", item.enclosure().unwrap().url().as_str());
        item_enclosure_tag.push_attribute(b"length",
                                          item.enclosure().unwrap().length().to_string().as_str());
        item_enclosure_tag.push_attribute(b"type", item.enclosure().unwrap().enclosure_type().as_str());
        writer.write(Start(item_enclosure_tag))
            .expect(errors::tag_start_error(item_enclosure_tag_str).as_str());

    }
}


fn write_item_guid(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.guid().is_some() {
        let item_guid_tag_str = "guid";
        let mut item_guid_tag = Element::new(item_guid_tag_str);
        item_guid_tag.push_attribute(b"isPermaLink",
                                     item.guid().unwrap().permalink().to_string().as_str());
        writer.write(Start(item_guid_tag))
            .expect(errors::tag_start_error(item_guid_tag_str).as_str());
        writer.write(Text(Element::new(item.guid().unwrap().guid().as_str())))
            .expect(errors::tag_text_error(item_guid_tag_str).as_str());
        writer.write(End(Element::new(item_guid_tag_str)))
            .expect(errors::tag_end_error(item_guid_tag_str).as_str());
    }
}


fn write_item_pub_date(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.pub_date().is_some() {
        let item_pub_date_tag_str = "pubDate";
        let item_pub_date_tag = Element::new(item_pub_date_tag_str);
        writer.write(Start(item_pub_date_tag))
            .expect(errors::tag_start_error(item_pub_date_tag_str).as_str());
        writer.write(Text(Element::new(item.pub_date().unwrap().to_rfc2822())))
            .expect(errors::tag_text_error(item_pub_date_tag_str).as_str());
        writer.write(End(Element::new(item_pub_date_tag_str)))
            .expect(errors::tag_end_error(item_pub_date_tag_str).as_str());
    }
}


fn write_item_source(mut writer: XmlWriter<Cursor<Vec<u8>>>, item: Item) {
    if item.source().is_some() {
        let item_source_tag_str = "source";
        let mut item_source_tag = Element::new(item_source_tag_str);
        item_source_tag.push_attribute(b"url", item.source().unwrap().url().to_string().as_str());
        writer.write(Start(item_source_tag))
            .expect(errors::tag_start_error(item_source_tag_str).as_str());
        writer.write(Text(Element::new(item.source().unwrap().source().as_str())))
            .expect(errors::tag_text_error(item_source_tag_str).as_str());
        writer.write(End(Element::new(item_source_tag_str)))
            .expect(errors::tag_end_error(item_source_tag_str).as_str());
    }
}


fn write_atom_feed(writer: XmlWriter<Cursor<Vec<u8>>>, feed_option: Option<AtomFeed>) {
    if feed_option.is_some() {
        let feed = feed_option.unwrap();

        write_feed_id(writer.clone(), feed.clone());
        write_feed_title(writer.clone(), feed.clone());
        write_feed_updated(writer.clone(), feed.clone());
        write_feed_authors(writer.clone(), feed.clone());
        write_feed_links(writer.clone(), feed.clone());
        write_feed_categories(writer.clone(), feed.clone());
        write_feed_contributors(writer.clone(), feed.clone());
        write_feed_generator(writer.clone(), feed.clone());
        write_feed_icon(writer.clone(), feed.clone());
        write_feed_logo(writer.clone(), feed.clone());
        write_feed_rights(writer.clone(), feed.clone());
        write_feed_subtitle(writer.clone(), feed.clone());
    }
}


fn write_feed_id(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    let id_tag_str = "atom:id";
    let id_tag = Element::new(id_tag_str);
    writer.write(Start(id_tag)).expect(errors::tag_start_error(id_tag_str).as_str());
    writer.write(Text(Element::new(feed.id().as_str())))
        .expect(errors::tag_text_error(id_tag_str).as_str());
    writer.write(End(Element::new(id_tag_str)))
        .expect(errors::tag_end_error(id_tag_str).as_str());
}


fn write_feed_title(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    let title = feed.clone().title();
    let title_tag_str = "atom:title";
    let mut title_tag = Element::new(title_tag_str);
    title_tag.push_attribute(b"type", title.text_type().as_str());
    writer.write(Start(title_tag)).expect(errors::tag_start_error(title_tag_str).as_str());
    writer.write(Text(Element::new(title.text().as_str())))
        .expect(errors::tag_text_error(title_tag_str).as_str());
    writer.write(End(Element::new(title_tag_str)))
        .expect(errors::tag_end_error(title_tag_str).as_str());
}


fn write_feed_updated(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    let updated_tag_str = "atom:updated";
    let updated_tag = Element::new(updated_tag_str);
    writer.write(Start(updated_tag)).expect(errors::tag_start_error(updated_tag_str).as_str());
    writer.write(Text(Element::new(feed.updated().to_rfc3339())))
        .expect(errors::tag_text_error(updated_tag_str).as_str());
    writer.write(End(Element::new(updated_tag_str)))
        .expect(errors::tag_end_error(updated_tag_str).as_str());
}


fn write_feed_authors(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().authors().is_some() {
        for author in feed.clone().authors().unwrap() {
            let author_tag_str = "atom:author";
            let author_tag = Element::new(author_tag_str);
            writer.write(Start(author_tag))
                .expect(errors::tag_start_error(author_tag_str).as_str());

            let name_tag_str = "atom:name";
            let name_tag = Element::new(name_tag_str);
            writer.write(Start(name_tag)).expect(errors::tag_start_error(name_tag_str).as_str());
            writer.write(Text(Element::new(author.name())))
                .expect(errors::tag_text_error(name_tag_str).as_str());
            writer.write(End(Element::new(name_tag_str)))
                .expect(errors::tag_end_error(name_tag_str).as_str());

            if author.email().is_some() {
                let email_tag_str = "atom:email";
                let email_tag = Element::new(email_tag_str);
                writer.write(Start(email_tag))
                    .expect(errors::tag_start_error(email_tag_str).as_str());
                writer.write(Text(Element::new(author.email().unwrap())))
                    .expect(errors::tag_text_error(email_tag_str).as_str());
                writer.write(End(Element::new(email_tag_str)))
                    .expect(errors::tag_end_error(email_tag_str).as_str());
            }

            if author.uri().is_some() {
                let uri_tag_str = "atom:uri";
                let uri_tag = Element::new(uri_tag_str);
                writer.write(Start(uri_tag))
                    .expect(errors::tag_start_error(uri_tag_str).as_str());
                writer.write(Text(Element::new(author.uri().unwrap().as_str())))
                    .expect(errors::tag_text_error(uri_tag_str).as_str());
                writer.write(End(Element::new(uri_tag_str)))
                    .expect(errors::tag_end_error(uri_tag_str).as_str());
            }

            writer.write(End(Element::new(author_tag_str)))
                .expect(errors::tag_end_error(author_tag_str).as_str());
        }
    }
}


fn write_feed_links(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().links().is_some() {
        for link in feed.clone().links().unwrap() {
            let link_tag_str = "atom:link";
            let mut link_tag = Element::new(link_tag_str);

            link_tag.push_attribute(b"href", link.href().as_str());

            if link.rel().is_some() {
                link_tag.push_attribute(b"rel", link.rel().unwrap().as_str());
            }

            if link.link_type().is_some() {
                link_tag.push_attribute(b"link", link.link_type().unwrap().as_str());
            }

            if link.href_lang().is_some() {
                link_tag.push_attribute(b"hreflang", link.href_lang().unwrap().as_str());
            }

            if link.title().is_some() {
                link_tag.push_attribute(b"title", link.title().unwrap().as_str());
            }

            if link.length().is_some() {
                link_tag.push_attribute(b"length", link.length().unwrap().to_string().as_str());
            }

            writer.write(Start(link_tag))
                .expect(errors::tag_start_error(link_tag_str).as_str());
            writer.write(End(Element::new(link_tag_str)))
                .expect(errors::tag_end_error(link_tag_str).as_str());
        }
    }
}


fn write_feed_categories(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().categories().is_some() {
        for category in feed.clone().categories().unwrap() {
            let category_tag_str = "atom:category";
            let mut category_tag = Element::new(category_tag_str);
            if category.scheme().is_some() {
                category_tag.push_attribute(b"scheme", category.scheme().unwrap().as_str());
            }
            if category.label().is_some() {
                category_tag.push_attribute(b"label", category.label().unwrap().as_str());
            }
            category_tag.push_attribute(b"term", category.term().as_str());
            writer.write(Start(category_tag))
                .expect(errors::tag_start_error(category_tag_str).as_str());
            writer.write(End(Element::new(category_tag_str)))
                .expect(errors::tag_end_error(category_tag_str).as_str());
        }
    }
}


fn write_feed_contributors(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().contributors().is_some() {
        for contributor in feed.clone().contributors().unwrap() {
            let contributor_tag_str = "atom:contributor";
            let contributor_tag = Element::new(contributor_tag_str);
            writer.write(Start(contributor_tag))
                .expect(errors::tag_start_error(contributor_tag_str).as_str());

            let name_tag_str = "atom:name";
            let name_tag = Element::new(name_tag_str);
            writer.write(Start(name_tag)).expect(errors::tag_start_error(name_tag_str).as_str());
            writer.write(Text(Element::new(contributor.name())))
                .expect(errors::tag_text_error(name_tag_str).as_str());
            writer.write(End(Element::new(name_tag_str)))
                .expect(errors::tag_end_error(name_tag_str).as_str());

            if contributor.email().is_some() {
                let email_tag_str = "atom:email";
                let email_tag = Element::new(email_tag_str);
                writer.write(Start(email_tag))
                    .expect(errors::tag_start_error(email_tag_str).as_str());
                writer.write(Text(Element::new(contributor.email().unwrap())))
                    .expect(errors::tag_text_error(email_tag_str).as_str());
                writer.write(End(Element::new(email_tag_str)))
                    .expect(errors::tag_end_error(email_tag_str).as_str());
            }

            if contributor.uri().is_some() {
                let uri_tag_str = "atom:uri";
                let uri_tag = Element::new(uri_tag_str);
                writer.write(Start(uri_tag))
                    .expect(errors::tag_start_error(uri_tag_str).as_str());
                writer.write(Text(Element::new(contributor.uri().unwrap().as_str())))
                    .expect(errors::tag_text_error(uri_tag_str).as_str());
                writer.write(End(Element::new(uri_tag_str)))
                    .expect(errors::tag_end_error(uri_tag_str).as_str());
            }

            writer.write(End(Element::new(contributor_tag_str)))
                .expect(errors::tag_end_error(contributor_tag_str).as_str());
        }
    }
}


fn write_feed_generator(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().generator().is_some() {
        let generator = feed.clone().generator().unwrap();
        let generator_tag_str = "atom:generator";
        let mut generator_tag = Element::new(generator_tag_str);

        if generator.uri().is_some() {
            generator_tag.push_attribute(b"uri", generator.uri().unwrap().as_str());
        }
        if generator.version().is_some() {
            generator_tag.push_attribute(b"version", generator.version().unwrap().as_str());
        }

        writer.write(Start(generator_tag))
            .expect(errors::tag_start_error(generator_tag_str).as_str());
        writer.write(Text(Element::new(generator.generator().as_str())))
            .expect(errors::tag_text_error(generator_tag_str).as_str());
        writer.write(End(Element::new(generator_tag_str)))
            .expect(errors::tag_end_error(generator_tag_str).as_str());
    }
}


fn write_feed_icon(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().icon().is_some() {
        let icon_tag_str = "atom:icon";
        let icon_tag = Element::new(icon_tag_str);
        writer.write(Start(icon_tag)).expect(errors::tag_start_error(icon_tag_str).as_str());
        writer.write(Text(Element::new(feed.icon().unwrap().as_str())))
            .expect(errors::tag_text_error(icon_tag_str).as_str());
        writer.write(End(Element::new(icon_tag_str)))
            .expect(errors::tag_end_error(icon_tag_str).as_str());
    }
}


fn write_feed_logo(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().logo().is_some() {
        let logo_tag_str = "atom:logo";
        let logo_tag = Element::new(logo_tag_str);
        writer.write(Start(logo_tag)).expect(errors::tag_start_error(logo_tag_str).as_str());
        writer.write(Text(Element::new(feed.logo().unwrap().as_str())))
            .expect(errors::tag_text_error(logo_tag_str).as_str());
        writer.write(End(Element::new(logo_tag_str)))
            .expect(errors::tag_end_error(logo_tag_str).as_str());
    }
}


fn write_feed_rights(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().rights().is_some() {
        let rights = feed.clone().rights().unwrap();
        let rights_tag_str = "atom:rights";
        let mut rights_tag = Element::new(rights_tag_str);
        rights_tag.push_attribute(b"type", rights.text_type().as_str());
        writer.write(Start(rights_tag)).expect(errors::tag_start_error(rights_tag_str).as_str());
        writer.write(Text(Element::new(rights.text().as_str())))
            .expect(errors::tag_text_error(rights_tag_str).as_str());
        writer.write(End(Element::new(rights_tag_str)))
            .expect(errors::tag_end_error(rights_tag_str).as_str());
    }
}


fn write_feed_subtitle(mut writer: XmlWriter<Cursor<Vec<u8>>>, feed: AtomFeed) {
    if feed.clone().subtitle().is_some() {
        let subtitle = feed.clone().subtitle().unwrap();
        let subtitle_tag_str = "atom:subtitle";
        let mut subtitle_tag = Element::new(subtitle_tag_str);
        subtitle_tag.push_attribute(b"type", subtitle.text_type().as_str());
        writer.write(Start(subtitle_tag))
            .expect(errors::tag_start_error(subtitle_tag_str).as_str());
        writer.write(Text(Element::new(subtitle.text().as_str())))
            .expect(errors::tag_text_error(subtitle_tag_str).as_str());
        writer.write(End(Element::new(subtitle_tag_str)))
            .expect(errors::tag_end_error(subtitle_tag_str).as_str());
    }
}


fn write_feed_entry_id(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    let id_tag_str = "atom:id";
    let id_tag = Element::new(id_tag_str);
    writer.write(Start(id_tag)).expect(errors::tag_start_error(id_tag_str).as_str());
    writer.write(Text(Element::new(entry.id().as_str())))
        .expect(errors::tag_text_error(id_tag_str).as_str());
    writer.write(End(Element::new(id_tag_str)))
        .expect(errors::tag_end_error(id_tag_str).as_str());
}


fn write_feed_entry_title(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    let title = entry.clone().title();
    let title_tag_str = "atom:title";
    let mut title_tag = Element::new(title_tag_str);
    title_tag.push_attribute(b"type", title.text_type().as_str());
    writer.write(Start(title_tag)).expect(errors::tag_start_error(title_tag_str).as_str());
    writer.write(Text(Element::new(title.text().as_str())))
        .expect(errors::tag_text_error(title_tag_str).as_str());
    writer.write(End(Element::new(title_tag_str)))
        .expect(errors::tag_end_error(title_tag_str).as_str());
}


fn write_feed_entry_updated(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    let updated_tag_str = "atom:updated";
    let updated_tag = Element::new(updated_tag_str);
    writer.write(Start(updated_tag)).expect(errors::tag_start_error(updated_tag_str).as_str());
    writer.write(Text(Element::new(entry.updated().to_rfc3339())))
        .expect(errors::tag_text_error(updated_tag_str).as_str());
    writer.write(End(Element::new(updated_tag_str)))
        .expect(errors::tag_end_error(updated_tag_str).as_str());
}


fn write_feed_entry_authors(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().authors().is_some() {
        for author in entry.clone().authors().unwrap() {
            let author_tag_str = "atom:author";
            let author_tag = Element::new(author_tag_str);
            writer.write(Start(author_tag))
                .expect(errors::tag_start_error(author_tag_str).as_str());

            let name_tag_str = "atom:name";
            let name_tag = Element::new(name_tag_str);
            writer.write(Start(name_tag)).expect(errors::tag_start_error(name_tag_str).as_str());
            writer.write(Text(Element::new(author.name())))
                .expect(errors::tag_text_error(name_tag_str).as_str());
            writer.write(End(Element::new(name_tag_str)))
                .expect(errors::tag_end_error(name_tag_str).as_str());

            if author.email().is_some() {
                let email_tag_str = "atom:email";
                let email_tag = Element::new(email_tag_str);
                writer.write(Start(email_tag))
                    .expect(errors::tag_start_error(email_tag_str).as_str());
                writer.write(Text(Element::new(author.email().unwrap())))
                    .expect(errors::tag_text_error(email_tag_str).as_str());
                writer.write(End(Element::new(email_tag_str)))
                    .expect(errors::tag_end_error(email_tag_str).as_str());
            }

            if author.uri().is_some() {
                let uri_tag_str = "atom:uri";
                let uri_tag = Element::new(uri_tag_str);
                writer.write(Start(uri_tag))
                    .expect(errors::tag_start_error(uri_tag_str).as_str());
                writer.write(Text(Element::new(author.uri().unwrap().as_str())))
                    .expect(errors::tag_text_error(uri_tag_str).as_str());
                writer.write(End(Element::new(uri_tag_str)))
                    .expect(errors::tag_end_error(uri_tag_str).as_str());
            }

            writer.write(End(Element::new(author_tag_str)))
                .expect(errors::tag_end_error(author_tag_str).as_str());
        }
    }
}


fn write_feed_entry_content(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().content().is_some() {
        let content = entry.clone().content().unwrap();
        let content_tag_str = "atom:content";
        let mut content_tag = Element::new(content_tag_str);
        content_tag.push_attribute(b"type", content.text_type().as_str());
        writer.write(Start(content_tag)).expect(errors::tag_start_error(content_tag_str).as_str());
        writer.write(Text(Element::new(content.text().as_str())))
            .expect(errors::tag_text_error(content_tag_str).as_str());
        writer.write(End(Element::new(content_tag_str)))
            .expect(errors::tag_end_error(content_tag_str).as_str());
    }
}


fn write_feed_entry_links(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().links().is_some() {
        for link in entry.clone().links().unwrap() {
            let link_tag_str = "atom:link";
            let mut link_tag = Element::new(link_tag_str);

            link_tag.push_attribute(b"href", link.href().as_str());

            if link.rel().is_some() {
                link_tag.push_attribute(b"rel", link.rel().unwrap().as_str());
            }

            if link.link_type().is_some() {
                link_tag.push_attribute(b"link", link.link_type().unwrap().as_str());
            }

            if link.href_lang().is_some() {
                link_tag.push_attribute(b"hreflang", link.href_lang().unwrap().as_str());
            }

            if link.title().is_some() {
                link_tag.push_attribute(b"title", link.title().unwrap().as_str());
            }

            if link.length().is_some() {
                link_tag.push_attribute(b"length", link.length().unwrap().to_string().as_str());
            }

            writer.write(Start(link_tag))
                .expect(errors::tag_start_error(link_tag_str).as_str());
            writer.write(End(Element::new(link_tag_str)))
                .expect(errors::tag_end_error(link_tag_str).as_str());
        }
    }
}


fn write_feed_entry_summary(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().summary().is_some() {
        let summary = entry.clone().summary().unwrap();
        let summary_tag_str = "atom:summary";
        let mut summary_tag = Element::new(summary_tag_str);
        summary_tag.push_attribute(b"type", summary.text_type().as_str());
        writer.write(Start(summary_tag)).expect(errors::tag_start_error(summary_tag_str).as_str());
        writer.write(Text(Element::new(summary.text().as_str())))
            .expect(errors::tag_text_error(summary_tag_str).as_str());
        writer.write(End(Element::new(summary_tag_str)))
            .expect(errors::tag_end_error(summary_tag_str).as_str());
    }
}


fn write_feed_entry_categories(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().categories().is_some() {
        for category in entry.clone().categories().unwrap() {
            let category_tag_str = "atom:category";
            let mut category_tag = Element::new(category_tag_str);
            if category.scheme().is_some() {
                category_tag.push_attribute(b"scheme", category.scheme().unwrap().as_str());
            }
            if category.label().is_some() {
                category_tag.push_attribute(b"label", category.label().unwrap().as_str());
            }
            category_tag.push_attribute(b"term", category.term().as_str());
            writer.write(Start(category_tag))
                .expect(errors::tag_start_error(category_tag_str).as_str());
            writer.write(End(Element::new(category_tag_str)))
                .expect(errors::tag_end_error(category_tag_str).as_str());
        }
    }
}


fn write_feed_entry_contributors(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().contributors().is_some() {
        for contributor in entry.clone().contributors().unwrap() {
            let contributor_tag_str = "atom:contributor";
            let contributor_tag = Element::new(contributor_tag_str);
            writer.write(Start(contributor_tag))
                .expect(errors::tag_start_error(contributor_tag_str).as_str());

            let name_tag_str = "atom:name";
            let name_tag = Element::new(name_tag_str);
            writer.write(Start(name_tag)).expect(errors::tag_start_error(name_tag_str).as_str());
            writer.write(Text(Element::new(contributor.name())))
                .expect(errors::tag_text_error(name_tag_str).as_str());
            writer.write(End(Element::new(name_tag_str)))
                .expect(errors::tag_end_error(name_tag_str).as_str());

            if contributor.email().is_some() {
                let email_tag_str = "atom:email";
                let email_tag = Element::new(email_tag_str);
                writer.write(Start(email_tag))
                    .expect(errors::tag_start_error(email_tag_str).as_str());
                writer.write(Text(Element::new(contributor.email().unwrap())))
                    .expect(errors::tag_text_error(email_tag_str).as_str());
                writer.write(End(Element::new(email_tag_str)))
                    .expect(errors::tag_end_error(email_tag_str).as_str());
            }

            if contributor.uri().is_some() {
                let uri_tag_str = "atom:uri";
                let uri_tag = Element::new(uri_tag_str);
                writer.write(Start(uri_tag))
                    .expect(errors::tag_start_error(uri_tag_str).as_str());
                writer.write(Text(Element::new(contributor.uri().unwrap().as_str())))
                    .expect(errors::tag_text_error(uri_tag_str).as_str());
                writer.write(End(Element::new(uri_tag_str)))
                    .expect(errors::tag_end_error(uri_tag_str).as_str());
            }

            writer.write(End(Element::new(contributor_tag_str)))
                .expect(errors::tag_end_error(contributor_tag_str).as_str());
        }
    }
}


fn write_feed_entry_published(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().published().is_some() {
        let published_tag_str = "atom:published";
        let published_tag = Element::new(published_tag_str);
        writer.write(Start(published_tag))
            .expect(errors::tag_start_error(published_tag_str).as_str());
        writer.write(Text(Element::new(entry.published().unwrap().to_rfc3339())))
            .expect(errors::tag_text_error(published_tag_str).as_str());
        writer.write(End(Element::new(published_tag_str)))
            .expect(errors::tag_end_error(published_tag_str).as_str());
    }
}


fn write_feed_entry_source(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().source().is_some() {
        let source_tag_str = "atom:source";
        let source_tag = Element::new(source_tag_str);
        writer.write(Start(source_tag))
            .expect(errors::tag_start_error(source_tag_str).as_str());
        writer.write(Text(Element::new(entry.source().unwrap())))
            .expect(errors::tag_text_error(source_tag_str).as_str());
        writer.write(End(Element::new(source_tag_str)))
            .expect(errors::tag_end_error(source_tag_str).as_str());
    }
}


fn write_feed_entry_rights(mut writer: XmlWriter<Cursor<Vec<u8>>>, entry: Entry) {
    if entry.clone().rights().is_some() {
        let rights = entry.clone().rights().unwrap();
        let rights_tag_str = "atom:rights";
        let mut rights_tag = Element::new(rights_tag_str);
        rights_tag.push_attribute(b"type", rights.text_type().as_str());
        writer.write(Start(rights_tag)).expect(errors::tag_start_error(rights_tag_str).as_str());
        writer.write(Text(Element::new(rights.text().as_str())))
            .expect(errors::tag_text_error(rights_tag_str).as_str());
        writer.write(End(Element::new(rights_tag_str)))
            .expect(errors::tag_end_error(rights_tag_str).as_str());
    }
}
