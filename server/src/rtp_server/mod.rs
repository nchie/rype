use std::io::{self, Cursor};

use futures::{Future, Async, Poll};

pub struct RtpServer {
}

impl RtpServer {
    pub fn new() -> Self {
        RtpServer { }
    }
}

impl Future for RtpServer {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        println!("asd");
        Ok(Async::Ready(()))
    }
}

