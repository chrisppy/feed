// Copyright (c) 2015-2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.


//! The fields under cloud can be retrieved by using the methods under `Cloud`.


use channels::Cloud;
use enums::CloudProtocol;
use url::Url;


impl Cloud {
    /// Get the domain that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    /// use feed::enums::CloudProtocol;
    ///
    /// let domain = "http://rpc.sys.com/";
    /// let cloud = CloudBuilder::new()
    ///     .domain(domain)
    ///     .protocol("soap")
    ///     .finalize();
    /// assert_eq!(domain.to_owned(), cloud.domain().into_string());
    /// ```
    pub fn domain(&self) -> Url {
        self.domain.clone()
    }


    /// Get the port that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    /// use feed::enums::CloudProtocol;
    ///
    /// let port: i64 = 80;
    /// let cloud = CloudBuilder::new()
    ///     .port(port)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize();
    /// assert_eq!(port, cloud.port());
    /// ```
    pub fn port(&self) -> i64 {
        self.port
    }


    /// Get the path that exists under `Cloud`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::channels::CloudBuilder;
    /// use feed::enums::CloudProtocol;
    ///
    /// let path = "/RPC2";
    /// let cloud = CloudBuilder::new()
    ///     .path(path)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
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
    /// use feed::channels::CloudBuilder;
    /// use feed::enums::CloudProtocol;
    ///
    /// let register_procedure = "pingMe";
    /// let cloud = CloudBuilder::new()
    ///     .register_procedure(register_procedure)
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
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
    /// use feed::channels::CloudBuilder;
    /// use feed::enums::CloudProtocol;
    ///
    /// let protocol = "soap";
    /// let cloud = CloudBuilder::new()
    ///     .protocol(protocol)
    ///     .domain("http://rpc.sys.com/")
    ///     .finalize();
    /// assert_eq!(protocol.to_owned(), cloud.protocol().into_string());
    /// ```
    pub fn protocol(&self) -> CloudProtocol {
        self.protocol.clone()
    }
}
