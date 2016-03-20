// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use chrono::*;
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
    let attr_result = str::from_utf8(attr[index]);
    match attr_result {
        Ok(result) => result,
        Err(err) => panic!("from utf8 error: {}", err),
    }
}


// Common code to convert attribute to i64.
pub fn attribute_to_i64(attributes: Attributes, index: usize) -> i64 {
    let attr = attributes.map(|a| a.unwrap().1).collect::<Vec<_>>();
    let attr_result = str::from_utf8(attr[index]);
    let attr_str = match attr_result {
        Ok(result) => result,
        Err(err) => panic!("from utf8 error: {}", err),
    };
    let i64_result = i64::from_str(attr_str);
    match i64_result {
        Ok(result) => result,
        Err(err) => panic!("from str error: {}", err),
    }
}

// Common code to convert attribute to bool.
pub fn attribute_to_bool(attributes: Attributes, index: usize) -> bool {
    let attr = attributes.map(|a| a.unwrap().1).collect::<Vec<_>>();
    let attr_result = str::from_utf8(attr[index]);
    let attr_str = match attr_result {
        Ok(result) => result,
        Err(err) => panic!("from utf8 error: {}", err),
    };
    let bool_result = bool::from_str(attr_str);
    match bool_result {
        Ok(result) => result,
        Err(err) => panic!("from str error: {}", err),
    }
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
    let result = e.into_string();
    match result {
        Ok(res) => res,
        Err(err) => panic!("Element into_string Error: {}", err),
    }
}


// Common code to convert element to Option<String>.
pub fn element_to_option_string(e: Element) -> Option<String> {
    Some(element_to_string(e))
}


// Common code to convert attribute to i64.
pub fn element_to_i64(e: Element) -> i64 {
    let e_string = element_to_string(e);
    let e_result = i64::from_str(&e_string);
    match e_result {
        Ok(result) => result,
        Err(err) => panic!("str to i64 error: {}", err),
    }
}


// Common code to convert attribute to Option<i64>.
pub fn element_to_option_i64(e: Element) -> Option<i64> {
    Some(element_to_i64(e))
}


// Common code to convert Option<String> to Option<DateTime<FixedOffset>>.
pub fn option_string_to_option_date(date_option: Option<String>) -> Option<DateTime<FixedOffset>> {
    let date_string = match date_option {
        Some(value) => value,
        None => {
            return None;
        }
    };
    let parsed_datetime = DateTime::parse_from_rfc2822(&date_string);
    match parsed_datetime {
        Ok(date) => Some(date),
        Err(err) => {
            panic!("DateTime Parse Error: {}", err);
        }
    }
}
