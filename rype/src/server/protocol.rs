use futures::{Async, Poll, future, Future, IntoFuture};
use tokio::io::{AsyncRead, AsyncWrite};
use super::service::{ Service, MakeService, Payload };

// TODO: Remove
pub struct Request {}

pub trait Protocol {
    // type Io: AsyncRead + AsyncWrite; 
    // type RequestFuture: Future;
    // type Request;
    
    // fn request(&mut self) -> Self::RequestFuture<Item=Self::Request, Error=()>;


}