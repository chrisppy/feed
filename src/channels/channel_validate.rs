// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! `Validate` Trait for `Channel`


use channels::Validate;
use rss::Channel;

impl Validate for Channel
{
    /// Validate `Channel`
    fn validate() -> Result<Channel, String>
    [
        unimplemented!()
    ]
}