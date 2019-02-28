
use crate::uri::Uri;
use crate::version::Version;
use crate::method::Method;

pub struct Request<T> {
    head: Parts,
    body: T

}

pub struct Parts {
    /// The request's method
    pub method: Method,

    /// The request's URI
    pub uri: Uri,

    /// The request's version
    pub version: Version,

    /// The request's headers
    // pub headers: HeaderMap<HeaderValue>,
    // TODO: Implement headers

    /// The request's extensions
    //pub extensions: Extensions,

    _priv: (),
}