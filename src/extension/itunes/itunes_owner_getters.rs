// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields under itunes category can be retrieved by using the methods under
//! `ITunesOwner`.


use extension::itunes::ITunesOwnerGetters;
use rss::extension::itunes::ITunesOwner;


impl ITunesOwnerGetters for ITunesOwner
{
    /// Get the optional name that exists under `ITunesOwner`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesOwnerBuilder, ITunesOwnerGetters};
    ///
    /// let name = "name".to_owned();
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .name(Some(name.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let name_opt = owner.name();
    /// assert!(name_opt.is_some());
    ///
    /// assert_eq!(name, name_opt.unwrap());
    /// ```
    ///
    // ```
    /// use feed::extension::itunes::{ITunesOwnerBuilder, ITunesOwnerGetters};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .name(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let name_opt = owner.name();
    /// assert!(name_opt.is_none());
    /// ```
    fn name(&self) -> Option<String>
    {
        self.name.clone()
    }


    /// Get the optional email that exists under `ITunesOwner`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::extension::itunes::{ITunesOwnerBuilder, ITunesOwnerGetters};
    ///
    /// let email = "email@example.com".to_owned();
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some(email.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let email_opt = owner.email();
    /// assert!(email_opt.is_some());
    ///
    /// assert_eq!(email, email_opt.unwrap());
    /// ```
    ///
    // ```
    /// use feed::extension::itunes::{ITunesOwnerBuilder, ITunesOwnerGetters};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let email_opt = owner.email();
    /// assert!(email_opt.is_none());
    /// ```
    fn email(&self) -> Option<String>
    {
        self.email.clone()
    }
}
