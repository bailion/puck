//! A HTTP library for the Lunatic Virtual Machine.

#![deny(missing_debug_implementations, unused_must_use)]

use std::{collections::HashMap, io::Write, net::ToSocketAddrs};

#[macro_use]
extern crate derivative;

#[cfg(test)]
mod regressions;

pub use puck_codegen::handler;

pub use anyhow;
pub use request::Request;
pub use response::Response;

use encoder::Encoder;
use request::{Body, HTML};

pub mod encoder;
pub mod request;
pub mod response;
pub mod ws;

pub trait Handler {
    fn handle<ADDRESS>(address: ADDRESS) -> anyhow::Result<()>
    where
        ADDRESS: ToSocketAddrs;
}

pub fn serve<H: Handler, ADDRESS: ToSocketAddrs>(address: ADDRESS) -> anyhow::Result<()> {
    H::handle(address)
}

pub fn err_404(_: Request) -> Response {
    Response {
        headers: {
            let mut res = HashMap::new();
            res.insert("Content-Type".to_string(), HTML.to_string());
            res
        },
        body: Body::from_string("<h1>404: Not found</h1>".to_string()),
        status: 404,
        reason: "not found".to_string(),
    }
}

pub fn err_400() -> Response {
    Response {
        headers: {
            let mut res = HashMap::new();
            res.insert("Content-Type".to_string(), HTML.to_string());
            res
        },
        body: Body::from_string("<h1>400: bad request</h1>".to_string()),
        status: 400,
        reason: "bad request".to_string(),
    }
}

pub fn write_response(res: Response, stream: impl Write) {
    let mut encoder = Encoder::new(res);
    encoder.write_tcp_stream(stream).unwrap();
}
