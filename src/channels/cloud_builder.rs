// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields can be set for cloud by using the methods under `CloudBuilder`.


use errors;
use channels::{Cloud, CloudBuilder};
use enums::CloudProtocol;
use utils::string_utils;


impl CloudBuilder {
    /// Construct a new `CloudBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    ///
    /// let cloud_builder = CloudBuilder::new();
    /// ```
    pub fn new() -> CloudBuilder {
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
    pub fn domain(&mut self, domain: &str) -> &mut CloudBuilder {
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
    pub fn port(&mut self, port: i64) -> &mut CloudBuilder {
        if port < 0 {
            panic!(errors::negative_error("cloud port", port));
        }
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
    pub fn path(&mut self, path: &str) -> &mut CloudBuilder {
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
    pub fn register_procedure(&mut self, register_procedure: &str) -> &mut CloudBuilder {
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
    pub fn protocol(&mut self, protocol: &str) -> &mut CloudBuilder {
        self.protocol = protocol.to_owned();
        self
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
    pub fn finalize(&self) -> Cloud {
        let domain_string = self.domain.clone();
        let domain = string_utils::str_to_url(domain_string.as_str(), "Cloud Domain");

        let protocol = CloudProtocol::to_enum(self.protocol.clone().as_str());

        Cloud {
            domain: domain,
            port: self.port,
            path: self.path.clone(),
            register_procedure: self.register_procedure.clone(),
            protocol: protocol,
        }
    }
}
