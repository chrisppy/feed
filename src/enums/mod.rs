// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! All of the enum.


/// Enumerations of protocols for `Cloud`.
#[derive(Clone, Debug)]
pub enum CloudProtocol
{
    /// http-post
    HttpPost,

    /// xml-rpc
    XmlRpc,

    /// soap
    Soap,
}


impl CloudProtocol
{
    /// Convert `&str` to `CloudProtocol`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::enums::CloudProtocol;
    ///
    /// let s = "soap";
    /// let e = CloudProtocol::value_of(s).unwrap();
    ///
    /// assert_eq!(s.to_owned(), e.into_string());
    /// ```
    pub fn value_of(s: &str) -> Result<CloudProtocol, String>
    {
        match s
        {
            "http-post" => Ok(CloudProtocol::HttpPost),
            "xml-rpc" => Ok(CloudProtocol::XmlRpc),
            "soap" => Ok(CloudProtocol::Soap),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }


    /// Convert `CloudProtocol` to `String`.
    pub fn into_string(self) -> String
    {
        match self
        {
            CloudProtocol::HttpPost => "http-post".to_owned(),
            CloudProtocol::XmlRpc => "xml-rpc".to_owned(),
            CloudProtocol::Soap => "soap".to_owned(),
        }
    }
}


/// Enumerations of protocols for `SkipDays`.
#[derive(Clone, Debug)]
pub enum Day
{
    /// Monday
    Monday,

    /// Tuesday
    Tuesday,

    /// Wednesday
    Wednesday,

    /// Thursday
    Thursday,

    /// Friday
    Friday,

    /// Saturday
    Saturday,

    /// Sunday
    Sunday,
}

impl Day
{
    /// Convert `&str` to `Day`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::enums::Day;
    ///
    /// let s = "Tuesday";
    /// let e = Day::value_of(s).unwrap();
    ///
    /// assert_eq!(s.to_owned(), e.into_string());
    /// ```
    pub fn value_of(s: &str) -> Result<Day, String>
    {
        match s
        {
            "Monday" => Ok(Day::Monday),
            "Tuesday" => Ok(Day::Tuesday),
            "Wednesday" => Ok(Day::Wednesday),
            "Thursday" => Ok(Day::Thursday),
            "Friday" => Ok(Day::Friday),
            "Saturday" => Ok(Day::Saturday),
            "Sunday" => Ok(Day::Sunday),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }


    /// Convert `Day` to `String`.
    pub fn into_string(self) -> String
    {
        match self
        {
            Day::Monday => "Monday".to_owned(),
            Day::Tuesday => "Tuesday".to_owned(),
            Day::Wednesday => "Wednesday".to_owned(),
            Day::Thursday => "Thursday".to_owned(),
            Day::Friday => "Friday".to_owned(),
            Day::Saturday => "Saturday".to_owned(),
            Day::Sunday => "Sunday".to_owned(),
        }
    }
}
