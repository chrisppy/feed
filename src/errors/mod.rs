// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! All the possible errors.

/// utf8 to str error.
pub fn utf8_to_str_error() -> &'static str {
    "Error converting utf8 to str"
}


/// vec get error
pub fn vec_get_error() -> &'static str {
    "Error executing get"
}


/// str to bool error.
pub fn str_to_bool_error() -> &'static str {
    "Error converting str to bool"
}


/// str to datetime error.
pub fn str_to_datetime_error() -> &'static str {
    "Error converting str to datetime"
}


/// str to i64 error.
pub fn str_to_i64_error() -> &'static str {
    "Error converting str to i64"
}


/// response error
pub fn response_error() -> &'static str {
    "Error retrieving response"
}


/// missing xml error
pub fn missing_xml_error() -> &'static str {
    "Url must end with .xml"
}


/// item required field error
pub fn item_required_field_error() -> &'static str {
    "Either Title or Description must have a value"
}


/// xml start tag error
pub fn tag_start_error(tag: &str) -> String {
    format!("Error creating start tag for {}", tag)
}


/// xml start tag error
pub fn tag_text_error(tag: &str) -> String {
    format!("Error creating text for {}", tag)
}


/// xml end tag error
pub fn tag_end_error(tag: &str) -> String {
    format!("Error creating end tag for {}", tag)
}


/// negative integer error
pub fn negative_error(tag: &str, num: i64) -> String {
    format!("{} cannot be negative: {}", tag, num)
}


/// invalid integer error
pub fn invalid_int_error(tag: &str, num: i64) -> String {
    format!("{} contains an invalid value: {}", tag, num)
}


/// invalid string error
pub fn invalid_str_error(tag: &str, string: &str) -> String {
    format!("{} contains an invalid value: {}", tag, string)
}


/// url parse error
pub fn url_parse_error(url: &str) -> String {
    format!("Error parsing url: {}", url)
}


/// date parse error
pub fn date_parse_error(url: &str) -> String {
    format!("Error parsing date: {}", url)
}
