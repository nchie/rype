use futures::{Stream, Future, future, Async, Poll};
use tokio::io::{AsyncRead, AsyncWrite};

use super::tcp::AddrIncoming;
use super::service::{ Service, MakeService, Payload };

pub struct Rtsp {
}

impl Rtsp {

}


pub struct Serve<I, MS> {
    incoming: I,
    make_service: MS
}

use super::codec::RtspCodec;

impl<I, MS, B> Stream for Serve<I, MS>
where
    I: Stream,
    I::Item: AsyncRead + AsyncWrite,
    MS: MakeService<I::Item, ReqBody=B, ResBody=B>,
    B: Payload
{
    type Item = Connection<MS::Service, I::Item, RtspCodec>;
    type Error = (); // TODO

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(io) = try_ready!(self.incoming.poll().map_err(|_|())) {
            let service = self.make_service.make_service().map_err(|_|())?;
            // Wrap io and service in an object which implements future, and uses the service to handle the request and response.
            let connection = Connection::new(io, service, RtspCodec{});
            Ok(Async::Ready(Some( connection )))
            //Ok(Async::Ready( Some(Connection { io: Some(io), service: service, proto: self.proto.clone() }) ))

        } else {
            Ok(Async::Ready(None))
        }
    }
}


enum Direction { Incoming, Outgoing }

pub struct Connection<S, I, C> {
    framed: Framed<I, C>,
    service: S,
    direction: Direction
}


use tokio::codec::{Decoder, Encoder, Framed, FramedParts};

impl<S, I, C> Connection<S, I, C> 
where
    S: Service,
    I: AsyncRead + AsyncWrite,
    C: Encoder + Decoder,
{
    fn new(io: I, service: S, codec: C) -> Connection<S, I, C>
    {
        let framed = codec.framed(io);
        Connection { framed, service, direction: Direction::Incoming }
    }

    // Change codec but keep buffers (i.e. when upgrading)
    fn into<NC>(self, new_codec: NC) -> Connection<S, I, NC>
    where
        NC: Encoder + Decoder
    {
        let parts = self.framed.into_parts();
        let mut new_parts = FramedParts::new(parts.io, new_codec);
        new_parts.read_buf = parts.read_buf;
        new_parts.write_buf = parts.write_buf;
        let framed = Framed::from_parts(new_parts);
        Connection { framed, service: self.service, direction: self.direction }
    }
}

impl<S, I, C> Future for Connection<S, I, C> 
where
    S: Service,
    I: AsyncRead + AsyncWrite,
    C: Encoder + Decoder
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        loop {
            let new_direction = match self.direction {
                Direction::Incoming => {
                    // Receive request
                    let message = match self.framed.poll() {
                        Ok(Async::Ready(value)) => value,
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(err) => return Err(()),
                    };

                    Direction::Outgoing
                },
                Direction::Outgoing => {
                    // Send response
                    Direction::Incoming
                }
            };

            self.direction = new_direction;
        }
    }
}