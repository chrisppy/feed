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
    // Convert `&str` to `CloudProtocol`.
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
}
