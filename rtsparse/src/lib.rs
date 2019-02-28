
pub struct Request<'headers, 'buf: 'headers> {
    pub method: Option<&'buf str>,
    pub path: Option<&'buf str>,
    pub version: Option<u8>,
    pub headers: &'headers mut [Header<'buf>]

}

impl<'h, 'b> Request<'h, 'b> {
    pub fn new(headers: &'h mut [Header<'b>]) -> Request<'h, 'b> {
        Request {
            method: None,
            path: None,
            version: None,
            headers: headers
        }
    }

    pub fn parse(&mut self, buf: &'b [u8]) -> Result<usize, ()> {
        // TODO: Parse
        Err(())
    }
}

pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a [u8]
}