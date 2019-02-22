use std::io::{self, Cursor};

use futures::{Future, Async, Poll};

pub struct RtcpServer {
}

impl RtcpServer {
    pub fn new() -> Self {
        RtpServer { }
    }
}

impl Future for RtcpServer {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        println!("asd");
        Ok(Async::Ready(()))
    }
}

