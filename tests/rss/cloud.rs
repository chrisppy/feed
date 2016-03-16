// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use feed::rss::cloud::CloudBuilder;

#[test]
fn domain() {
    let domain = "rpc.sys.com";
    let cloud = CloudBuilder::new()
        .domain(domain)
        .finalize();
    assert_eq!(domain.to_string(), cloud.domain());
}


#[test]
fn port() {
    let port: i64 = 80;
    let cloud = CloudBuilder::new()
        .port(port)
        .finalize();
    assert_eq!(port, cloud.port());
}


#[test]
fn path() {
    let path = "/RPC2";
    let cloud = CloudBuilder::new()
        .path(path)
        .finalize();
    assert_eq!(path.to_string(), cloud.path());
}


#[test]
fn register_procedure() {
    let register_procedure = "pingMe";
    let cloud = CloudBuilder::new()
        .register_procedure(register_procedure)
        .finalize();
    assert_eq!(register_procedure.to_string(), cloud.register_procedure());
}


#[test]
fn protocol() {
    let protocol = "soap";
    let cloud = CloudBuilder::new()
        .protocol(protocol)
        .finalize();
    assert_eq!(protocol.to_string(), cloud.protocol());
}
