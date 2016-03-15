// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

#[derive(Clone)]
pub struct Cloud {
    domain:             String,
    port:               i64,
    path:               String,
    register_procedure: String,
    protocol:           String,
}


impl Cloud {
    pub fn domain(&self) -> String {
        self.domain.clone()
    }


    pub fn port(&self) -> i64 {
        self.port.clone()
    }


    pub fn path(&self) -> String {
        self.path.clone()
    }


    pub fn register_procedure(&self) -> String {
        self.register_procedure.clone()
    }


    pub fn protocol(&self) -> String {
        self.protocol.clone()
    }



}


pub struct CloudBuilder {
    domain:             String,
    port:               i64,
    path:               String,
    register_procedure: String,
    protocol:           String,
}


impl CloudBuilder {
    pub fn new() -> CloudBuilder {
        CloudBuilder {
            domain:             String::new(),
            port:               0,
            path:               String::new(),
            register_procedure: String::new(),
            protocol:           String::new(),
        }
    }


    pub fn domain(&mut self, domain: &str) -> &mut CloudBuilder {
        self.domain = domain.to_string();
        self
    }


    pub fn port(&mut self, port: i64) -> &mut CloudBuilder {
        self.port = port;
        self
    }


    pub fn path(&mut self, path: &str) -> &mut CloudBuilder {
        self.path = path.to_string();
        self
    }


    pub fn register_procedure(&mut self, register_procedure: &str) -> &mut CloudBuilder {
        self.register_procedure = register_procedure.to_string();
        self
    }


    pub fn protocol(&mut self, protocol: &str) -> &mut CloudBuilder {
        self.protocol = protocol.to_string();
        self
    }


    pub fn finalize(&self) -> Cloud {
        Cloud {
            domain:             self.domain.clone(),
            port:               self.port.clone(),
            path:               self.path.clone(),
            register_procedure: self.register_procedure.clone(),
            protocol:           self.protocol.clone(),
        }
    }
}
