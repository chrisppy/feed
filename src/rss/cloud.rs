// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

//! The fields under cloud can be retrieved by using the methods under `Cloud`
//! and the fields can be set for cloud by using the methods under `CloudBuilder`.

/// This `Cloud` struct contains all the items that exist for the cloud field under 'Channel'.
#[derive(Clone)]
pub struct Cloud {
    domain: String,
    port: i64,
    path: String,
    register_procedure: String,
    protocol: String,
}


impl Cloud {
    /// Get the domain that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let domain = "rpc.sys.com";
    /// let cloud = CloudBuilder::new()
    ///     .domain(domain)
    ///     .finalize();
    /// assert_eq!(domain.to_owned(), cloud.domain());
    /// ```
    pub fn domain(&self) -> String {
        self.domain.clone()
    }


    /// Get the port that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let port: i64 = 80;
    /// let cloud = CloudBuilder::new()
    ///     .port(port)
    ///     .finalize();
    /// assert_eq!(port, cloud.port());
    /// ```
    pub fn port(&self) -> i64 {
        self.port.clone()
    }


    /// Get the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let path = "/RPC2";
    /// let cloud = CloudBuilder::new()
    ///     .path(path)
    ///     .finalize();
    /// assert_eq!(path.to_owned(), cloud.path());
    /// ```
    pub fn path(&self) -> String {
        self.path.clone()
    }


    /// Get the register procedure that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let register_procedure = "pingMe";
    /// let cloud = CloudBuilder::new()
    ///     .register_procedure(register_procedure)
    ///     .finalize();
    /// assert_eq!(register_procedure.to_owned(), cloud.register_procedure());
    /// ```
    pub fn register_procedure(&self) -> String {
        self.register_procedure.clone()
    }


    /// Get the protocol that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let protocol = "soap";
    /// let cloud = CloudBuilder::new()
    ///     .protocol(protocol)
    ///     .finalize();
    /// assert_eq!(protocol.to_owned(), cloud.protocol());
    /// ```
    pub fn protocol(&self) -> String {
        self.protocol.clone()
    }
}


/// This `CloudBuilder` struct creates the `Cloud`.
#[derive(Default)]
pub struct CloudBuilder {
    domain: String,
    port: i64,
    path: String,
    register_procedure: String,
    protocol: String,
}


impl CloudBuilder {
    /// Construct a new `CloudBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
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
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.domain("rpc.sys.com");
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
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let mut cloud_builder = CloudBuilder::new();
    /// cloud_builder.port(80);
    /// ```
    pub fn port(&mut self, port: i64) -> &mut CloudBuilder {
        self.port = port;
        self
    }


    /// Set the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::rss::cloud::CloudBuilder;
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
    /// use feed::rss::cloud::CloudBuilder;
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
    /// use feed::rss::cloud::CloudBuilder;
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
    /// use feed::rss::cloud::CloudBuilder;
    ///
    /// let cloud = CloudBuilder::new()
    ///         .domain("rpc.sys.com")
    ///         .port(80)
    ///         .path("/RPC2")
    ///         .register_procedure("pingMe")
    ///         .protocol("soap")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Cloud {
        Cloud {
            domain: self.domain.clone(),
            port: self.port.clone(),
            path: self.path.clone(),
            register_procedure: self.register_procedure.clone(),
            protocol: self.protocol.clone(),
        }
    }
}
