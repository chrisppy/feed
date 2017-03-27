// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for itunes category by using the methods under
//! `ITunesOwnerBuilder`.


use channels::itunes::{ITunesOwner, ITunesOwnerBuilder};


impl ITunesOwnerBuilder
{
    ///
    pub fn new() -> ITunesOwnerBuilder
    {
        ITunesOwnerBuilder::default()
    }


    ///
    pub fn name(&mut self, name: Option<String>) -> &mut ITunesOwnerBuilder
    {
        self.name = name;
        self
    }


    ///
    pub fn email(&mut self, email: Option<String>) -> &mut ITunesOwnerBuilder
    {
        self.email = email;
        self
    }


    ///
    pub fn finalize(&self) -> Result<ITunesOwner, String>
    {
        Ok(ITunesOwner {
               name: self.name.clone(),
               email: self.email.clone(),
           })
    }
}
