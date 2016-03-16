// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
use quick_xml::attributes::Attributes;
use std::{i64, str};
use std::str::FromStr;

pub fn str_to_option_string(s: &str) -> Option<String> {
    let string = s.to_string();
    Some(string)
}


pub fn content_to_str(content: &[u8]) -> &str {
    let content_str = str::from_utf8(content).unwrap();
    content_str
}


pub fn content_to_i64(content: &[u8]) -> i64 {
    let content_str = content_to_str(content);
    let content_i64 = i64::from_str(content_str).unwrap();
    content_i64
}

pub fn content_to_bool(content: &[u8]) -> bool {
    let content_str = content_to_str(content);
    let content_bool = bool::from_str(content_str).unwrap();
    content_bool
}


pub fn content_to_option_string(content: &[u8]) -> Option<String> {
    let content_str = content_to_str(content);
    str_to_option_string(content_str)
}


pub fn content_to_option_i64(content: &[u8]) -> Option<i64> {
    let content_str = content_to_str(content);
    let content_i64 = i64::from_str_radix(content_str, 10).unwrap();
    Some(content_i64)
}


pub fn attribute_to_str(attributes: Attributes, index: usize) -> &str {
    let attr = attributes.map(|a| a.unwrap().1).collect::<Vec<_>>();
    let attr_str = content_to_str(attr[index]);
    attr_str
}


pub fn attribute_to_i64(attributes: Attributes, index: usize) -> i64 {
    let attr = attributes.map(|a| a.unwrap().1).collect::<Vec<_>>();
    let attr_i64 = content_to_i64(attr[index]);
    attr_i64
}


pub fn attribute_to_bool(attributes: Attributes, index: usize) -> bool {
    let attr = attributes.map(|a| a.unwrap().1).collect::<Vec<_>>();
    let attr_bool = content_to_bool(attr[index]);
    attr_bool
}


pub fn attribute_to_option_string(attributes: Attributes, index: usize) -> Option<String> {
    let attr_str = attribute_to_str(attributes, index);
    str_to_option_string(attr_str)
}


pub fn attribute_to_option_bool(attributes: Attributes, index: usize) -> Option<bool> {
    let attr_bool = attribute_to_bool(attributes, index);
    Some(attr_bool)
}


pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    if date_option.is_none() {
        return None;
    }
    let date_string = date_option.unwrap();
    let parsed_datetime = DateTime::parse_from_rfc2822(&date_string);
    let datetime = match parsed_datetime {
        Ok(date) => date,
        Err(err) => {
            panic!("Error: {}", err);
        },
    };
    Some(datetime)
}
