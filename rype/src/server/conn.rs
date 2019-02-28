use futures::{Stream, Future, future, Async, Poll};
use tokio::io::{AsyncRead, AsyncWrite};

use super::tcp::AddrIncoming;
use super::service::{ Service, MakeService, Payload };
use super::protocol::Protocol;

pub struct Rtsp {
}

impl Rtsp {

}

struct DummyProtocol<I>{
    io: I
}

// use super::protocol::Request;
// impl<I, F, R> Protocol for DummyProtocol<I>
// where
//     I: AsyncRead + AsyncWrite,
//     F: Future<Item=Request, Error=()>
// {
//     type Io = I;

//     fn request(&mut self) -> F {
//         future::err::<Request, ()>(())
//     }
// }

pub struct Serve<I, MS, P> {
    incoming: I,
    make_service: MS,
    proto: P
}

impl<I, MS, B, P> Stream for Serve<I, MS, P>
where
    I: Stream,
    I::Item: AsyncRead + AsyncWrite,
    MS: MakeService<I::Item, ReqBody=B, ResBody=B>,
    B: Payload,
    P: Protocol + Clone
{
    type Item = Connection<I::Item, MS::Service, P>;
    type Error = (); // TODO

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(io) = try_ready!(self.incoming.poll().map_err(|_|())) {
            let service = self.make_service.make_service().map_err(|_|())?;
            // TODO: Wrap io and service in an object which implements future, and uses the service to handle the request and response.
            Ok(Async::Ready( Some(Connection { io: Some(io), service: service, proto: self.proto.clone() }) ))

        } else {
            Ok(Async::Ready(None))
        }
    }
}


pub struct Connection<I, S, P> {
    io: Option<I>,
    service: S,
    proto: P
}

impl<I, S, P> Future for Connection<I, S, P> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // TODO: While not shutdown
        // TODO: Handle keep-alive
        // 1. Use `self.proto` to get `Request` from `self.io`
        // 2. Poll request thorugh self.service.call
        // 3. Return response to self.io
        Ok(Async::Ready( () ))
    }
}