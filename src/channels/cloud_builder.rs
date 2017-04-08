// This file is part of feed.
//
// Copyright © 2015-2017 Chris Palmer <pennstate5013@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.


//! The fields can be set for cloud by using the methods under `CloudBuilder`.


use channels::CloudBuilder;
use enums::CloudProtocol;
use rss::Cloud;
use utils::string_utils;


impl CloudBuilder
{
    /// Construct a new `CloudBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let cloud_builder = CloudBuilder::new();
    /// ```
    pub fn new() -> CloudBuilder
    {
        CloudBuilder::default()
    }


    /// Set the domain that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.domain("http://rpc.sys.com/");
    /// ```
    pub fn domain(&mut self, domain: &str) -> &mut CloudBuilder
    {
        self.domain = domain.to_owned();
        self
    }


    /// Set the port that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.port(80);
    /// ```
    pub fn port(&mut self, port: i64) -> &mut CloudBuilder
    {

        self.port = port;
        self
    }


    /// Set the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.path("/RPC2");
    /// ```
    pub fn path(&mut self, path: &str) -> &mut CloudBuilder
    {
        self.path = path.to_owned();
        self
    }


    /// Set the register procedure that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.register_procedure("pingMe");
    /// ```
    pub fn register_procedure(&mut self, register_procedure: &str) -> &mut CloudBuilder
    {
        self.register_procedure = register_procedure.to_owned();
        self
    }


    /// Set the protocol that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.protocol("soap");
    /// ```
    pub fn protocol(&mut self, protocol: &str) -> &mut CloudBuilder
    {
        self.protocol = protocol.to_owned();
        self
    }


    /// Validate the contents of `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new()
    ///         .domain("http://rpc.sys.com/")
    ///         .port(80)
    ///         .path("/RPC2")
    ///         .register_procedure("pingMe")
    ///         .protocol("soap")
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(&mut self) -> Result<&mut CloudBuilder, String>
    {
        if self.port < 0
        {
            return Err("Cloud Port cannot be a negative value".to_owned());
        }

        string_utils::str_to_url(self.domain.as_str())?;
        CloudProtocol::value_of(self.protocol.as_str())?;

        Ok(self)
    }


    /// Construct the `Cloud` from the `CloudBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new()
    ///         .domain("http://rpc.sys.com/")
    ///         .port(80)
    ///         .path("/RPC2")
    ///         .register_procedure("pingMe")
    ///         .protocol("soap")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Result<Cloud, String>
    {
        let port = string_utils::i64_to_string(self.port)?;

        Ok(Cloud {
               domain: self.domain.clone(),
               port: port,
               path: self.path.clone(),
               register_procedure: self.register_procedure.clone(),
               protocol: self.protocol.clone(),
           })
    }
}
