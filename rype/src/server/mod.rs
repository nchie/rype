pub mod conn;
pub mod tcp;
pub mod session;
pub mod service;
mod codec;


use std::io::{self, Cursor};

use futures::{Stream, Future, Async, Poll};
use tokio::net::TcpListener;
use std::net::SocketAddr;

pub struct Builder {

}


impl Builder {
    pub fn serve(self) -> Result<Server, ()>{
        let addr = "127.0.0.1:8080".to_string();
        let addr = addr.parse::<SocketAddr>().map_err(|_err| ())?;

        let server = Server {
            //listener: TcpListener::bind(&addr).map_err(|err| ())?
        };

        Ok(server)

    }
}

pub struct Server {
    //listener: TcpListener
}

impl Server {
    pub fn builder() -> Builder {
        Builder {}
    }

}

impl Future for Server {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        println!("asd");
        Ok(Async::Ready(()))
    }
}

