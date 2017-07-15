// Copyright 2017 Rapidity Networks, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate chrono;
extern crate hyper;
extern crate hyper_openssl;
extern crate openssl;
#[macro_use]
extern crate percent_encoding;
extern crate rand;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate xml;

mod application;
mod client;
pub mod encoding;
mod error;
mod oauth;
mod resources;

pub use application::{Application, PrivateApplication};
pub use client::Client;
pub use openssl::rsa::Rsa;
pub use openssl::pkey::PKey;
pub mod accounting {
    pub use resources::contacts::*;
    pub use resources::invoices::*;
    pub use resources::items::*;
    pub use resources::payments::*;
}
