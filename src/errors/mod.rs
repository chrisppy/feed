// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


// utf8 to str error.
pub fn utf8_to_str_error() -> String {
    "Error converting utf8 to str".to_owned()
}


// str to datetime error.
pub fn str_to_datetime_error() -> String {
    "Error converting str to datetime".to_owned()
}


// str to i64 error.
pub fn str_to_i64_error() -> String {
    "Error converting str to i64".to_owned()
}


// response error
pub fn missing_xml_error() -> String {
    "Url must end with .xml".to_owned()
}


// item required field error
pub fn item_required_field_error() -> String {
    "Either Title or Description must have a value".to_owned()
}


// negative integer error
pub fn negative_error(tag: &str, num: i64) -> String {
    format!("{} cannot be negative: {}", tag, num)
}


// invalid integer error
pub fn invalid_int_error(tag: &str, num: i64) -> String {
    format!("{} contains an invalid value: {}", tag, num)
}


// invalid string error
pub fn invalid_str_error(tag: &str, string: &str) -> String {
    format!("{} contains an invalid value: {}", tag, string)
}


// content type error
pub fn content_type_error() -> String {
    "Error retrieving content type".to_owned()
}


// image url error
pub fn image_url_error() -> String {
    "Image Url must end with .jpeg, .png, or .gif".to_owned()
}


// string to url error
pub fn str_to_url_error() -> String {
    "Error parsing str into url".to_owned()
}


// url error
pub fn url_error() -> String {
    "Url error".to_owned()
}
