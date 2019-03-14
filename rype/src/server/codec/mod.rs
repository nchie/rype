use std::io;

use tokio::codec::Framed;
use tokio::codec::{Decoder, Encoder};
use bytes::BytesMut;


pub struct RtspRequest;
pub struct RtspResponse;

pub struct RtspCodec;


impl Decoder for RtspCodec
{
    type Item = RtspRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<RtspRequest>> {
        // TODO: Read and parse `src` to a Request
        Ok(Some((RtspRequest{})))
    }
}

impl Encoder for RtspCodec
{
    type Item = RtspResponse;
    type Error = io::Error;

    fn encode(&mut self, item: RtspResponse, dst: &mut BytesMut) -> io::Result<()> {
        // TODO: Encode `item` to `dst` and return Ok(()) when finished.
        Ok(())
    }
}
