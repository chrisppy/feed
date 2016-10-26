// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The feed can be converted to xml.


pub mod reader_utils;
pub mod writer_utils;


use chrono::*;
use errors;
use quick_xml::attributes::Attributes;
use quick_xml::Element;
use std::{i64, str};
use std::str::FromStr;


// Common code to convert &str to Option<String>.
pub fn str_to_option_string(s: &str) -> Option<String> {
    Some(s.to_owned())
}


// Common code to convert attribute to &str.
pub fn attribute_to_str(attributes: Attributes, index: usize) -> &str {
    let attr = attributes.map(|a| a.unwrap().1).collect::<Vec<_>>();
    str::from_utf8(attr[index]).expect(errors::utf8_to_str_error().as_str())
}


// Common code to convert attribute to i64.
pub fn attribute_to_i64(attributes: Attributes, index: usize) -> i64 {
    let attr_str = attribute_to_str(attributes, index);
    i64::from_str(attr_str).expect(errors::str_to_i64_error().as_str())
}

// Common code to convert attribute to bool.
pub fn attribute_to_bool(attributes: Attributes, index: usize) -> bool {
    let attr_str = attribute_to_str(attributes, index);
    bool::from_str(attr_str).expect(errors::str_to_bool_error().as_str())
}


// Common code to convert attribute to Option<String>.
pub fn attribute_to_option_string(attributes: Attributes, index: usize) -> Option<String> {
    str_to_option_string(attribute_to_str(attributes, index))
}


// Common code to convert attribute to Option<bool>.
pub fn attribute_to_option_bool(attributes: Attributes, index: usize) -> Option<bool> {
    Some(attribute_to_bool(attributes, index))
}


// Common code to convert element to String.
pub fn element_to_string(e: Element) -> String {
    e.into_string().expect(errors::element_to_string_error().as_str())
}


// Common code to convert element to Option<String>.
pub fn element_to_option_string(e: Element) -> Option<String> {
    Some(element_to_string(e))
}


// Common code to convert attribute to i64.
pub fn element_to_i64(e: Element) -> i64 {
    let e_string = element_to_string(e);
    i64::from_str(&e_string).expect(errors::str_to_i64_error().as_str())
}


// Common code to convert attribute to Option<i64>.
pub fn element_to_option_i64(e: Element) -> Option<i64> {
    Some(element_to_i64(e))
}


// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    if date_option.is_none() {
        return None;
    }
    let date_string = date_option.unwrap();
    let datetime = DateTime::parse_from_rfc2822(&date_string)
        .expect(errors::str_to_datetime_error().as_str());
    Some(datetime)
}
