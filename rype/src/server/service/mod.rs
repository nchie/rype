use std::error::Error as StdError;
use futures::{Async, Poll, future, Future, IntoFuture};

pub trait Payload {}
pub struct Request<T>(T);
pub struct Response<T>(T);
impl Payload for () {}

/// An asynchronous function from `Request` to `Response`.
pub trait Service {
    /// The `Payload` body of the `http::Request`.
    type ReqBody: Payload;

    /// The `Payload` body of the `http::Response`.
    type ResBody: Payload;

    /// The error type that can occur within this `Service.
    ///
    /// Note: Returning an `Error` to a hyper server will cause the connection
    /// to be abruptly aborted. In most cases, it is better to return a `Response`
    /// with a 4xx or 5xx status code.
    type Error: Into<Box<StdError + Send + Sync>>;

    /// The `Future` returned by this `Service`.
    type Future: Future<Item=Response<Self::ResBody>, Error=Self::Error>;

    /// Calls this `Service` with a request, returning a `Future` of the response.
    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future;
}


/// A constructor of `Service`s.
pub trait MakeService<Ctx> {
    /// The `Payload` body of the `http::Request`.
    type ReqBody: Payload;

    /// The `Payload` body of the `http::Response`.
    type ResBody: Payload;

    /// The error type that can be returned by `Service`s.
    type Error: Into<Box<StdError + Send + Sync>>;

    /// The resolved `Service` from `new_service()`.
    type Service: Service<
        ReqBody=Self::ReqBody,
        ResBody=Self::ResBody,
        Error=Self::Error,
    >;

    /// The error type that can be returned when creating a new `Service`.
    type MakeError: Into<Box<StdError + Send + Sync>>;

    /// Returns `Ready` when the constructor is ready to create a new `Service`.
    ///
    /// The implementation of this method is allowed to return a `Ready` even if
    /// the factory is not ready to create a new service. In this case, the future
    /// returned from `make_service` will resolve to an error.
    fn poll_ready(&mut self) -> Poll<(), Self::MakeError> {
        Ok(Async::Ready(()))
    }

    /// Create a new `Service`.
    fn make_service(&mut self) -> Result<Self::Service, Self::MakeError>;
}