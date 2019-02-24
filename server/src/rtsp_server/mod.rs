mod conn;
mod tcp;

use std::io::{self, Cursor};

use futures::{Stream, Future, Async, Poll};
use tokio::net::TcpListener;
use std::net::SocketAddr;

pub struct RtspServerBuilder {

}


impl RtspServerBuilder {
    pub fn serve(self) -> Result<RtspServer, ()>{
        let addr = "127.0.0.1:8080".to_string();
        let addr = addr.parse::<SocketAddr>().map_err(|err| ())?;

        let server = RtspServer {
            //listener: TcpListener::bind(&addr).map_err(|err| ())?
        };

        Ok(server)

    }
}

pub struct RtspServer {
    //listener: TcpListener
}

impl RtspServer {
    pub fn builder() -> RtspServerBuilder {
        RtspServerBuilder {}
    }

}

impl Future for RtspServer {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        println!("asd");
        Ok(Async::Ready(()))
    }
}

