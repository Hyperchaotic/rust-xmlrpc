// Copyright 2014-2015 Galen Clark Haynes
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Rust XML-RPC library

use hyper;
use std::io::Read;
use super::{Request, Response};

pub struct Client {
    url: String,
}

// The Client in rust-xmlrpc can panic. Using own implementation here.
impl Client {
    pub fn new(s: &str) -> Client {
        Client { url: s.to_string() }
    }

    pub fn remote_call(&self, request: &Request) -> Result<Response, hyper::Error> {
        let mut http_client = hyper::Client::new();
        let mut result = try!(http_client.post(&self.url).body(&request.body).send());

        let mut body = String::new();

        try!(result.read_to_string(&mut body));
        Ok(Response::new(&body))
    }
}
